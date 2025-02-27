# Leafish
**This project was built upon preexisting code located here: https://github.com/iceiix/stevenarella/**

But why creating a new project in the first place..?

Because I wanted to remove some stuff and add other stuff freely without having to wait for prs to get accepted.

Multi-protocol Minecraft-compatible client written in Rust. It's fast.

If you want to communicate with us you can do so here:
https://matrix.to/#/#leafish:matrix.org
https://discord.gg/YkHCy3f4qt

## Images

![Steven on Hypixel](https://i.imgur.com/PM5fLuu.png)
![Steven](https://i.imgur.com/RRspOQF.png)


In action: http://gfycat.com/NeedyElaborateGypsymoth

## Speed comparison

(screenshots or some kind of comparison here)


## Protocol support

| Game version | Protocol version | Supported? |
| ------ | --- | --- |
| 1.16.5 | 754 | ✓ |
| 1.16.4 | 754 | ✓ |
| 1.16.3 | 753 | ✓ |
| 1.16.2 | 751 | ✓ |
| 1.16.1 | 736 | ✓ |
| 1.16 | 735 | ✓ |
| 1.15.2 | 578 | ✓ |
| 1.15.1 | 575 | ✓ |
| 1.14.4 | 498 | ✓ |
| 1.14.3 | 490 | ✓ |
| 1.14.2 | 485 | ✓ |
| 1.14.1 | 480 | ✓ |
| 1.14 | 477 | ✓ |
| 19w02a | 452 | ✓ |
| 18w50a | 451 | ✓ |
| 1.13.2 | 404 | ✓ |
| 1.12.2 | 340 | ✓ |
| 1.11.2 | 316 | ✓ |
| 1.11   | 315 | ✓ |
| 1.10.2 | 210 | ✓ |
| 1.9.2  | 109 | ✓ |
| 1.9    | 107 | ✓ |
| 15w39c | 74  | ✓ |
| 1.8.9  | 47  | ✓ |
| 1.7.10 | 5   | ✓ |

Leafish is designed to support multiple protocol versions, so that client
development is not in lock-step with the server version. The level of
support varies, but the goal is to support major versions from 1.7.10
up to the current latest major version. Occasionally, snapshots are also supported.

Forge servers are supported on 1.7.10 - 1.12.2 (FML) and 1.13.2 - 1.16.5 (FML2).

Support for older protocols will _not_ be dropped as newer protocols are added.

## Dependencies

Requires Rust stable version 1.54.0 or newer.

**Debian/Ubuntu**

```bash
sudo apt-get install libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxcb-composite0-dev
```

**Alpine Linux**

```bash
sudo apk add openssl-dev xcb-util-dev
```

## Building

Compile and run:
```bash
cargo run --release
```
Just compile:
```bash
cargo build --release
```

## Running

### Standalone

Just running Leafish via a double click (Windows) or `./leafish` (everything else)
will bring up a login screen followed by a server list which you can select a server
from.

## Contributions

Leafish is an [OPEN Open Source Project](https://github.com/openopensource/openopensource.github.io):

> Individuals making significant and valuable contributions are given
> commit-access to the project to contribute as they see fit. This project
> is more like an open wiki than a standard guarded open source project.

### Rules

There are a few basic ground-rules for contributors:

1. **No `--force` pushes** or modifying the Git history on the `master` branch.
1. **Non-master branches** ought to be used for ongoing work.
1. **External API changes and significant modifications** ought to be subject to an **internal pull-request** to solicit feedback from other contributors.
1. Internal pull-requests to solicit feedback are *encouraged* for any other non-trivial contribution but left to the discretion of the contributor.
1. Contributors should attempt to adhere to the prevailing code-style. Please install and run [cargo fmt](https://github.com/rust-lang/rustfmt) before merging any changes.

### Changes to this arrangement

This is an experiment and feedback is welcome! This document may also be
subject to pull-requests or changes by contributors where you believe
you have something valuable to add or change.

### Credits

Thanks to [@thinkofname](https://github.com/thinkofname/) for
the original [Steven (Rust)](https://github.com/thinkofname/steven),
which Stevenarella is an updated and enhanced version of.

### License

Dual-licensed MIT and ApacheV2
