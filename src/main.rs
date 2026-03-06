use anyhow::{bail, Context, Result};
use clap::Parser;
use kaspa_addresses::{Address, Prefix, Version};
use rand::rngs::ThreadRng;
use secp256k1::{Keypair, Secp256k1, SecretKey, XOnlyPublicKey};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const BECH32_CHARS: &str = "qpzry9x8gf2tvdw0s3jn54khce6mua7l";

#[derive(Parser, Debug, Clone)]
#[command(
    name = "kaspa-vanity",
    about = "Fast multithreaded vanity Kaspa address generator"
)]
struct Args {
    /// Prefix to match in the payload part after "kaspa:"
    #[arg(short, long, default_value = "")]
    prefix: String,

    /// Suffix to match at the end of the payload part
    #[arg(short, long, default_value = "")]
    suffix: String,

    /// Require prefix to match after the initial 'q' in the payload
    #[arg(long, default_value_t = true)]
    after_q: bool,

    /// Number of worker threads
    #[arg(short, long)]
    threads: Option<usize>,

    /// Print every generated address
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Output JSON file
    #[arg(short, long, default_value = "kaspa_vanity_result.json")]
    output: PathBuf,
}

#[derive(Debug, Serialize, Clone)]
struct VanityResult {
    address: String,
    payload: String,
    private_key_hex: String,
    xonly_public_key_hex: String,
    attempts: u64,
    elapsed_seconds: f64,
    threads: usize,
    prefix: String,
    suffix: String,
    after_q: bool,
}

fn validate_pattern(label: &str, s: &str) -> Result<()> {
    if s.chars().all(|c| BECH32_CHARS.contains(c)) {
        Ok(())
    } else {
        bail!(
            "invalid {} '{}'. Allowed chars: {}",
            label,
            s,
            BECH32_CHARS
        )
    }
}

fn build_address(secret_key: &SecretKey) -> (String, String, String) {
    let secp = Secp256k1::new();
    let keypair = Keypair::from_secret_key(&secp, secret_key);
    let (xonly_pubkey, _) = XOnlyPublicKey::from_keypair(&keypair);

    let payload = xonly_pubkey.serialize();
    let address = Address::new(Prefix::Mainnet, Version::PubKey, &payload);
    let address_string: String = address.into();

    (
        address_string,
        hex::encode(secret_key.secret_bytes()),
        hex::encode(payload),
    )
}

fn payload_part(address: &str) -> &str {
    address.split_once(':').map(|(_, p)| p).unwrap_or(address)
}

fn is_match(payload: &str, prefix: &str, suffix: &str, after_q: bool) -> bool {
    let prefix_ok = if prefix.is_empty() {
        true
    } else if after_q {
        let wanted = format!("q{}", prefix);
        payload.starts_with(&wanted)
    } else {
        payload.starts_with(prefix)
    };

    let suffix_ok = if suffix.is_empty() {
        true
    } else {
        payload.ends_with(suffix)
    };

    prefix_ok && suffix_ok
}

fn random_secret_key(rng: &mut ThreadRng) -> SecretKey {
    SecretKey::new(rng)
}

fn main() -> Result<()> {
    let args = Args::parse();

    validate_pattern("prefix", &args.prefix)?;
    validate_pattern("suffix", &args.suffix)?;

    let threads = args.threads.unwrap_or_else(num_cpus::get).max(1);

    let stop = Arc::new(AtomicBool::new(false));
    let attempts = Arc::new(AtomicU64::new(0));
    let winner: Arc<Mutex<Option<VanityResult>>> = Arc::new(Mutex::new(None));

    {
        let stop = stop.clone();
        ctrlc::set_handler(move || {
            eprintln!("\nCtrl+C received, stopping...");
            stop.store(true, Ordering::Relaxed);
        })
        .context("failed to install Ctrl+C handler")?;
    }

    let started = Instant::now();

    println!(
        "Searching mainnet Kaspa vanity address | prefix='{}' suffix='{}' after_q={} threads={}",
        args.prefix, args.suffix, args.after_q, threads
    );

    let mut handles = Vec::with_capacity(threads);

    for _ in 0..threads {
        let args = args.clone();
        let stop = stop.clone();
        let attempts = attempts.clone();
        let winner = winner.clone();
        let started = started;

        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();

            while !stop.load(Ordering::Relaxed) {
                let sk = random_secret_key(&mut rng);
                let (address, private_key_hex, xonly_public_key_hex) = build_address(&sk);
                let payload = payload_part(&address).to_string();
                let n = attempts.fetch_add(1, Ordering::Relaxed) + 1;

                if args.verbose {
                    println!("{}", address);
                }

                if is_match(&payload, &args.prefix, &args.suffix, args.after_q) {
                    let result = VanityResult {
                        address,
                        payload,
                        private_key_hex,
                        xonly_public_key_hex,
                        attempts: n,
                        elapsed_seconds: started.elapsed().as_secs_f64(),
                        threads,
                        prefix: args.prefix.clone(),
                        suffix: args.suffix.clone(),
                        after_q: args.after_q,
                    };

                    let mut guard = winner.lock().unwrap();
                    if guard.is_none() {
                        *guard = Some(result);
                        stop.store(true, Ordering::Relaxed);
                    }
                    break;
                }
            }
        });

        handles.push(handle);
    }

    let mut last_attempts = 0u64;
    while !stop.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_secs(1));
        let total = attempts.load(Ordering::Relaxed);
        let delta = total.saturating_sub(last_attempts);
        last_attempts = total;
        let elapsed = started.elapsed().as_secs_f64();
        let rate = if elapsed > 0.0 {
            total as f64 / elapsed
        } else {
            0.0
        };
        eprint!(
            "\rTried {:>12} | {:>10.0} addr/s total | {:>8} addr/s last sec",
            total, rate, delta
        );
    }
    eprintln!();

    for handle in handles {
        let _ = handle.join();
    }

    let guard = winner.lock().unwrap();
    if let Some(result) = guard.clone() {
        let json = serde_json::to_string_pretty(&result)?;
        fs::write(&args.output, json).with_context(|| {
            format!("failed to write result file {}", args.output.display())
        })?;

        println!("Found match!");
        println!("Address:          {}", result.address);
        println!("Private key hex:  {}", result.private_key_hex);
        println!("XOnly pubkey hex: {}", result.xonly_public_key_hex);
        println!("Attempts:         {}", result.attempts);
        println!("Elapsed:          {:.2} s", result.elapsed_seconds);
        println!("Saved to:         {}", args.output.display());
    } else {
        println!("Stopped without finding a match.");
    }

    Ok(())
}
