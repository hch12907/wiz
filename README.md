# Wiz
[![Build Status](https://travis-ci.org/hch12907/wiz.svg?branch=rewrite)](https://travis-ci.org/hch12907/wiz)

> Wiz is a package installer/manager and launcher, written in pure Rust (with no bindings).

Wiz started out as a tool aiming to ease the installation of libraries and binaries.

## Goals for the rewrite
The old code contained a lot of hardcoding, and the whole thing was just too messy to be looked at. This rewrite tries to organize the code properly: minimizing the amount of hardcoding, to pave the road for future expansion of the program. More features of Rust should be utilized to achieve this, as the old code utilized only macros and structs.

<!--
## Installation
The Wiz project maintains multiple types of releases:
 * *Current:* Released from rapid development branches of this repository.
* *LTS:* Releases that receive Long-Term Support, versioned by SemVer, and signed by member of team. Will release binary files on github. 

### Download
Binaries, installer and source tarballs are available at <GITHUB_RELEASES>
Binaries: <snip>
Installer: <snip>
Source: <snip>
-->

## Building Wiz
If you have already installed [Rust **nightly** and Cargo](https://www.rust-lang.org/en-US/install.html), then simply run these commands at any given directory (with permissions, of course) :
```
git clone https://github.com/hch12907/wiz
cd wiz
cargo build
```

## Contributors
* [hch12907](https://github.com/hch12907)
* [zypeh](https://github.com/zypeh)
