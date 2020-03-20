# Locate

[![GitHub issues](https://img.shields.io/github/issues/pr4k/locate)](https://github.com/pr4k/locate/issues)
[![GitHub forks](https://img.shields.io/github/forks/pr4k/locate)](https://github.com/pr4k/locate/network)
[![GitHub stars](https://img.shields.io/github/stars/pr4k/locate)](https://github.com/pr4k/locate/stargazers)
[![Github action](https://github.com/pr4k/locate/workflows/Publish/badge.svg)](https://github.com/pr4k/locate/workflows/Publish/badge.svg)
[![Rustc 1.42](https://img.shields.io/badge/rustc-1.42-red.svg)](https://img.shields.io/badge/rustc-1.42-red.svg)

How many times have you gone through files to find some string, or maybe want to know where you have used a name.

Locate is a tool written in *Rust* which goes through each file and gives exact location of the occurence of a name or string in the file.

---

## Installation 

``` bash
# Clone the repo
git clone https://github.com/pr4k/locate

# Build the project
cargo build --release

```
It will create a binary in `target/release`, copy it to `/usr/bin` and you are good to go.

---

## Give it a Test Run

Don't want to clone the project, *No worries!!* 

Go to the releases and downlaod the latest release, copy it to `/usr/bin` that's all.

---

## Usage

```bash
➜  ~ locate -h                       
Usage:
  locate [OPTIONS]

Recursive string locater in files

Optional arguments:
  -h,--help             Show this help message and exit
  -p,--path PATH        Path to folder
  -q,--query QUERY      Query string to find
```
Try `locate -p /path/to/folder -q /string-to-be-searched`


[![asciicast](https://asciinema.org/a/9uBSxwooJEGBLNZvoaW5zPPhF.svg)](https://asciinema.org/a/9uBSxwooJEGBLNZvoaW5zPPhF)

---

## TO-Do

- Add regex support for the search string
- Control Folder depth for search
- Provide option to replace the searched word by another
  
## License
[![GitHub license](https://img.shields.io/github/license/pr4k/locate)](https://github.com/pr4k/locate)

- **[MIT license](http://opensource.org/licenses/mit-license.php)**
- Copyright 2020 © pr4k
---