# Kaspa Address Generator with Prefix/Suffix Search
This project is a Python-based tool that allows users to generate Kaspa cryptocurrency addresses with specific prefix and suffix patterns using the offline `kaspaper` package from GitHub (`github.com/svarogg/kaspaper`). The tool generates addresses rapidly, providing detailed output and optional verbose logs, and saves the results to an HTML file.

## Features:
- **Prefix search**: Specify a custom prefix after the `kaspa:q` part of the address.
- **Suffix search**: Specify a custom suffix at the end of the address.
- **Verbose mode**: Output every generated address to the console.
- **Fast address generation**: Uses an offline tool for rapid address creation.
- **Automatic saving**: Found addresses are saved into an HTML file.
- **Graceful exit**: Interrupting the script (Ctrl+C) will delete the temporary files and exit cleanly.

---

## Prerequisites

1. **Python 3.8+**: This script requires Python 3.8 or later. Install Python if you don't already have it:
   ```bash
   sudo apt-get install python3
   ```
   or, for macOS:
   ```bash
   brew install python3
   ```

2. **kaspaper binary**: Download the `kaspaper` binary from the [official repository](https://github.com/svarogg/kaspaper/) or compile it from the source. Ensure that the binary is in the same directory as the Python script.

3. **Python packages**:
   Install the necessary Python packages using `pip`:
   ```bash
   pip3 install argparse shutil / pip install argparse shutil
   ```

---

## Usage

### Command-Line Options

The script accepts several options for customizing the address search:

- `-p`, `--prefix`: Specify a prefix to match after `kaspa:q`.
- `-s`, `--suffix`: Specify a suffix to match at the end of the address.
- `-v`, `--verbose`: Enable verbose output of every generated address.

### Examples

1. **Basic address generation**:
   ```bash
   python3 kaspa_address_generator.py
   ```
   This generates a random Kaspa address and saves it in the format `kaspa:<address>.html`.

2. **Prefix and suffix search**:
   ```bash
   python3 kaspa_address_generator.py -p p -s jj
   ```
   This searches for an address that starts with `qp` (after `kaspa:q`) and ends with `jj`. Once an address is found, it is saved in `qp...jj.html`.

3. **Verbose mode**:
   ```bash
   python3 kaspa_address_generator.py -v
   ```
   This generates addresses and outputs each address to the console until an address is found (if prefix/suffix is specified).

4. **Graceful exit**:
   To stop the script, press `Ctrl+C`. This will delete any temporary files and exit cleanly.

---

## Example Output

If you run the script with:
```bash
python3 kaspa_address_generator.py -p p -s jj
```

The output will be:
```
Searching for addresses with prefix 'p' and suffix 'jj'...
Tested 2580 addresses in 22.5 minutes.
Success! Address found: kaspa:q**p**zry9x...*jj* after 2580 attempts in 22.5 minutes.
Result saved to qp...jj.html
```

---

## Files in the Repository

1. **kaspa_address_generator.py**: The main Python script for generating Kaspa addresses with prefix/suffix options.
2. **kaspaper**: The binaries of Kaspaper from https://github.com/svarogg/kaspaper/releases/ (kaspaper-linux, kaspaper-mac, and kaspaper-windows.exe; the python script automatically defines the OS type and uses the right binary.
3. **README.md**: This documentation file.
4. **requirements.txt**: List of required Python packages.

---

## Future Enhancements

- Add support for multi-core processing to speed up the address generation process.
- Provide a user-friendly GUI for specifying prefix/suffix and monitoring progress.

## Support Project
If you liked or found this project useful, donations are welcome to: `kaspa:qr62af6d77jvqpdrgmsgcrf7d2vgp2z9h4dyxvq6eqxe0jg4qs3dxm04a0000`
