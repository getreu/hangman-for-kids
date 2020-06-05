---
title: ASCII-Hangman - hangman game for children with ASCII-art rewarding
---

[![Cargo](https://img.shields.io/crates/v/ascii-hangman.svg)](
https://crates.io/crates/ascii-hangman)
[![Documentation](https://docs.rs/ascii-hangman/badge.svg)](
https://docs.rs/ascii-hangman)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](
https://github.com/getreu/ascii-hangman)

A little game designed for primary kids to revise vocabulary in classroom. Setting
up the vocabulary list is part of the learning process.

Hangman is a paper and pencil guessing game for two or more players.  One player
thinks of a word, phrase or sentence and the other tries to guess it by
suggesting letters or numbers, within a certain number of guesses. In this
version for children the computer selects a word, phrase or sentence randomly
out of a word-list defined in a configuration file. As a reward, with every
guessed letter, some ASCII-art is gradually disclosed.

```
ASCII-ART HANGMAN FOR KIDS

          ,.
         (_|,.
        ,' /, )_____
     __j o``-'
    (")
     `-j
       `-._(
          |_\  |--^.
         /_]'|_| /_
            /_]'  /

Lives:	7	Last guess: k

 g o o d   l u _ k

Type a letter, then press [Enter]: 
```

Read more in [ASCII-Hangman's user manual](/projects/ascii-hangman/ascii-hangman--manual.html).

## Documentation

User documentation:

* User manual:

  [ASCII-Hangman user manual - html](/projects/ascii-hangman/ascii-hangman--manual.html)

  [ASCII-Hangman user manual - pdf](/_downloads/ascii-hangman--manual.pdf)


Developer documentation:

* [API documentation](/projects/ascii-hangman/_downloads/doc/ascii_hangman/)


## Source code

Repository:

* [ASCII-Hangman on Github](https://github.com/getreu/ascii-hangman)


## Distribution

* Binaries for latest release (Linux, Windows, iOS)

    1. Open: [Releases - getreu/ascii-hangman](https://github.com/getreu/ascii-hangman/releases)

    2. Open the latest release.

    3. Open *assets*.

    4. Download the packed executable for your operating system.

    5. Installation: see below.

* Binaries and packages (usually built from latest commit):

  - Executable for Windows:

    [x86_64-pc-windows-gnu/release/ascii-hangman.exe](/projects/ascii-hangman/_downloads/x86_64-pc-windows-gnu/release/ascii-hangman.exe)

  - Binary for Linux:

    [x86_64-unknown-linux-gnu/release/ascii-hangman](/projects/ascii-hangman/_downloads/x86_64-unknown-linux-gnu/release/ascii-hangman)

    [x86_64-unknown-linux-musl/release/ascii-hangman](/projects/ascii-hangman/_downloads/x86_64-unknown-linux-musl/release/ascii-hangman)

  - Package for Debian and Ubuntu:

    [x86_64-unknown-linux-gnu/debian/ascii-hangman_4.10.1_amd64.deb](/projects/ascii-hangman/_downloads/x86_64-unknown-linux-gnu/debian/ascii-hangman_4.10.1_amd64.deb)


* Zipfile with all binaries and documentation:

  - [ascii-hangman all](/_downloads/ascii-hangman.zip)



## Building and installing

1. [Install Rust](https://www.rust-lang.org/tools/install), e.g.

       curl https://sh.rustup.rs -sSf | sh

2. Download, compile and install:

       cargo install ascii-hangman
       sudo cp ~/.cargo/bin/ascii-hangman /usr/local/bin

   See also the user manual for a more detailed installation description.

This project follows [Semantic Versioning](https://semver.org/).



## About

Author:

* Jens Getreu

Copyright:

* Apache 2 licence or MIT licence

<!--
Build status:

* ![status](https://travis-ci.org/getreu/ascii-hangman.svg?branch=master)  
-->