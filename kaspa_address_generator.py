import subprocess
import time
import argparse
import sys
import os
import shutil
import random
import string
import platform

# Allowed Bech32 characters
allowed_chars = 'qpzry9x8gf2tvdw0s3jn54khce6mua7l'

# Function to generate a random filename for tmp.html
def generate_random_filename():
    return ''.join(random.choices(string.ascii_lowercase + string.digits, k=8)) + '.html'

# Function to get the appropriate kaspaper binary based on the operating system
def get_kaspaper_command():
    system = platform.system()
    if system == 'Linux':
        return './kaspaper-linux'
    elif system == 'Darwin':  # macOS is detected as 'Darwin'
        return './kaspaper-mac'
    elif system == 'Windows':
        return 'kaspaper.exe'  # Windows uses .exe extension
    else:
        print(f"Unsupported operating system: {system}")
        sys.exit(1)

# Function to run the kaspaper command
def run_kaspaper_command(temp_filename):
    kaspaper_command = get_kaspaper_command()
    
    system = platform.system()
    
    # Use 'nul' for Windows and '/dev/null' for Unix-like systems
    if system == 'Windows':
        command = f'{kaspaper_command} {temp_filename}'
        with open('nul', 'w') as devnull:
            subprocess.run(command, shell=True, stdout=devnull, stderr=devnull)
    else:
        command = f'{kaspaper_command} {temp_filename}'
        with open('/dev/null', 'w') as devnull:
            subprocess.run(command, shell=True, stdout=devnull, stderr=devnull)

# Function to read and extract the address from HTML
def extract_address_from_html(temp_filename):
    with open(temp_filename, 'r') as file:
        content = file.read()

    # Find the address within the div class="addr"
    start = content.find('<div class="addr">')
    if start == -1:
        return None
    start += len('<div class="addr">')
    end = content.find('</div>', start)

    # Clean up the address (remove <br>, newlines, spaces)
    address = content[start:end].replace('<br>', '').replace('\n', '').replace(' ', '').strip()
    return address

# Function to validate prefix and suffix
def validate_input(input_string):
    return all(c in allowed_chars for c in input_string)

# Main function to generate addresses based on user inputs
def main():
    parser = argparse.ArgumentParser(description="Kaspa Address Generator with Prefix/Suffix Search")
    parser.add_argument("-p", "--prefix", type=str, help="Prefix to match after 'kaspa:q'", default="")
    parser.add_argument("-s", "--suffix", type=str, help="Suffix to match at the end of the address", default="")
    parser.add_argument("-v", "--verbose", action="store_true", help="Enable verbose output of every generated address")

    args = parser.parse_args()
    prefix = args.prefix
    suffix = args.suffix
    verbose = args.verbose

    # Validate the prefix and suffix
    if prefix and not validate_input(prefix):
        print(f"Invalid prefix: {prefix}. Allowed characters: {allowed_chars}")
        sys.exit(1)

    if suffix and not validate_input(suffix):
        print(f"Invalid suffix: {suffix}. Allowed characters: {allowed_chars}")
        sys.exit(1)

    attempts = 0
    start_time = time.time()

    print(f"Searching for addresses with prefix '{prefix}' and suffix '{suffix}'...")
    temp_filename = generate_random_filename()

    try:
        while True:
            run_kaspaper_command(temp_filename)
            address = extract_address_from_html(temp_filename)
            attempts += 1

            # Remove the 'kaspa:' part from the address for prefix/suffix matching
            clean_address = address.replace('kaspa:', '')

            # Verbose output for each generated address
            if verbose:
                print(f"Generated address: {address}")

            # Ensure prefix matches only after "q" in "kaspa:q..."
            if clean_address.startswith(f"q{prefix}") and clean_address.endswith(suffix):
                break

            # Every 60 seconds, update the statistics
            if attempts % 10 == 0:
                elapsed_time = int(time.time() - start_time) / 60
                print(f"Tested {attempts} addresses in {elapsed_time:.2f} minutes.", end='\r')

            # If no prefix/suffix is provided, stop after the first generated address
            if not args.prefix and not args.suffix:
                break

        # Create the result filename and move the original HTML file to this new name
        if args.prefix or args.suffix:
            result_filename = f"q{prefix}...{suffix}.html"
        else:
            result_filename = f"{address}.html"

        # Move temp file to the new file name
        shutil.move(temp_filename, result_filename)

        elapsed_time = int(time.time() - start_time) / 60
        print(f"\nSuccess! Address found: {address} after {attempts} attempts in {elapsed_time:.2f} minutes.")
        print(f"Result saved to {result_filename}")

    except KeyboardInterrupt:
        print("\nProcess interrupted by user. Cleaning up...")
        if os.path.exists(temp_filename):
            os.remove(temp_filename)
        print(f"Temporary file '{temp_filename}' deleted.")
        sys.exit(0)

if __name__ == "__main__":
    main()
