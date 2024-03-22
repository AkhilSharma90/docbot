<div align="center">
  <h1><code>DOCBOT</code></h1>
  <p><strong>A PDF password cracking tool with multi-threading capabilities, featuring password format generators for commonly used patterns and dictionary attack functionalities.</strong>< p>
</div>

INSPIRED FROM OffensiveRust (want to explore weaponization of Rust, check this repo out - https://github.com/trickster0/OffensiveRust) and also a lot FROM THE BOOK - BLACK HAT WITH RUST

For Educational Purposes Only. (or is it? :p)

Name DocBot - inspired from matrix sentinel (also called machine, docbot)

![LLM Discord Bot: matrix-sentinel](images/matrix-sentinel.jpg)

## 📖 Table of Contents

- [Introduction](#%E2%84%B9%EF%B8%8F-introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Contribution](#contribution)
- [License](#license)

## ℹ️ Introduction


**DOCBOT** is a swift, multithreaded PDF password cracking tool crafted in Rust. It supports dictionary attacks based on wordlists, as well as brute-forcing techniques like date, number range, and alphanumeric combinations. Additionally, it includes a personalized query builder for defining specific password formats.

## Features

- **Custom Query Builder:** Allows for the creation of personalized queries such as `STRING{69-420}`, which generates and utilizes a wordlist with the entire specified number range.
- **Date Bruteforce:** Enables bruteforce attempts for all 365 days of a given year in `DDMMYYYY` format, commonly encountered in PDF password formats.
- **Number Bruteforce:** Supports bruteforce attacks within a specified number range, e.g., `5000-100000`.
- **Default Bruteforce:** Allows the specification of maximum and optionally minimum password lengths for a search. All passwords within the defined length range (from 4 up to the specified maximum) consisting of letters and numbers (`a-zA-Z0-9`) will be attempted.
- **Fast:** Achieves speeds of approximately 50k-100k+ passwords per second, utilizing all available CPU cores.


## Installation

Install with `cargo`:

    $ cargo install --git https://github.com/akhilsharma90/docbot.git
    
[Install Rust/Cargo](https://rust-lang.org/tools/install)

## Build From Source

**Prerequisites:**

* [Git](https://git-scm.org/downloads)
* [Rust](https://rust-lang.org/tools/install)
* Cargo (Automatically installed when installing Rust)
* A C linker (Only for Linux, generally comes pre-installed)

```
$ git clone https://github.com/akhilsharma90/docbot.git
$ cd docbot/
$ cargo build --release
```

The first command clones this repository into your local machine and the last two commands enters the directory and builds the source in release mode.

## Usage

Get a list of all the arguments:

    $ docbot --help
    
Start a dictionary attack with a wordlist:

    $ docbot -f encrypted.pdf wordlist rockyou.txt
    
Bruteforce number ranges for the password:

    $ docbot -f encrypted.pdf range 1000 9999
    
Bruteforce all dates in a span (inclusive in both ends) of years for the password in `DDMMYYYY` format:

    $ docbot -f encrypted.pdf date 1900 2000

Bruteforce arbitrary strings of length 4-8:

    $ docbot -f encrypted.pdf default-query --max-length 8

Bruteforce arbitrary strings of length 3:

    $ docbot -f encrypted.pdf default-query --max-length 3 --min-length 3

Build a custom query to generate a wordlist: (useful when you know the password format)

    $ docbot -f encrypted.pdf custom-query ALICE{1000-9999}

    $ docbot -f encrypted.pdf custom-query DOC-ID{0-99}-FILE

Enable preceding zeros for custom queries: (which would make `{10-5000}` to `{0010-5000}` matching the end range's digits)

    $ docbot -f encrypted.pdf custom-query ALICE{10-9999} --add-preceding-zeros

## Contribution

Ways to contribute:

- Suggest a feature
- Report a bug
- Fix something and open a pull request
- Help me document the code
- Spread the word

## License

Licensed under the MIT License, see <a href="https://github.com/akhilsharma90/docbot/blob/master/LICENSE">LICENSE</a> for more information.
