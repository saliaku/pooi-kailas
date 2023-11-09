![](./pooi.png)
- trivia on the command line

![define_ah](https://files.mastodon.social/media_attachments/files/111/377/440/489/281/999/original/da4f92e91b4ea911.png)

use ```pooi --help``` for full usage information

## dependencies:

on all platforms the only thing you need installed is cargo.

+ Windows: https://win.rustup.rs/
+ Linux/macOS: ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh``` (or use your distro's package manager)

this is only required for building the binary, if you have no more use for cargo you can remove it afterwards

## build instructions:

clone repo

cd into repo root

#### Linux
an install script has been provided, just run ```./install.sh```

#### macOS
run ```cargo build --release```

then ```sudo cp ./target/release/pooi /usr/local/bin```

a zsh completion script is located at the following path ```./etc/completions/_oi``` but I currently have no idea where to put it (sorry!)

#### Windows
run ```cargo build --release```

then make a new folder in a location of your choosing

```copy .\target\release\pooi.exe [letter]:\path\to\your\folder```

following [this](https://medium.com/@kevinmarkvi/how-to-add-executables-to-your-path-in-windows-5ffa4ce61a53) guide you can add your new folder to you environment PATH

a PowerShell completion script is located at the following path ```.\etc\completions\_oi.ps1``` but I currently have no idea where to put it (sorry!)

### Pre-Compiled version for Linux is available in the Release section. You may download that and run it using ./pooi or move it to your $PATH

### TODO:

- [x] shell completion scripts
- [x] add a proper release with binaries
- [x] license?
- [ ] improve docs
- [ ] user customisable colours (using an environment variable)
- [ ] general code improvements (and probable bug fixes)

### TO-DO List @tellmeY18
- [ ] package it for cargo
- [ ] package for debian
- [x] add to AUR
- [ ] package for Arch mebbe ?
