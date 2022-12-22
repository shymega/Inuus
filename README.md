# Inuus: Guardian of your SSH keys

Welcome to the README for "Inuus", the protector of your SSH keys.

## Overview

Inuus is a work-in-progress at the moment, and borrows some ideas from [`passh-agent`][passh-agent], another SSH agent written in Rust.

However, Inuus deviates in the sense that we're looking to support multiple backends, including [`pass`][pass], [Bitwarden][], and [1Password][]. We aim to be a dynamic, fast, and above all, *secure* SSH key management tool for multiple platforms.

## Status of project

Keep an eye on this repository, work is continuing behind the scenes, and CI nightlies are coming soon, as well as a Nix package definition.

## Found a vulnerability?

There should be an option to privately report security vulnerabilities on this repository. If that doesn't work for you, email this address, using my GPG key (you can find it on [Keybase][]): `shymega+inuus.vulns AT shymega DOT org DOT uk`.

## License

Licensed under the GPLv3 (or later version). The license file can be found [here][License].

[passh-agent]: https://github.com/MrPixelized/passh-agent
[pass]: https://www.passwordstore.org/
[Bitwarden]: https://bitwarden.com/
[1Password]: https://1password.com/
[Keybase]: https://keybase.io/shymega
[License]: /LICENSE
