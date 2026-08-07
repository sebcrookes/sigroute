# Sigroute

Sigroute is a modern Linux-native automation system written in Rust. It provides functionality to run commands, scripts, backups, and other tasks based on triggers, all set up through a modern libadwaita-compliant GUI. It consists of the front-end GUI application, which communicates with a daemon (sigrouted) over D Bus. The daemon manages a SQLite database and running the automations.

Please note: Not all features are currently present - this application is under development

[![Rust CI](https://github.com/sebcrookes/sigroute/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/sebcrookes/sigroute/actions/workflows/rust-ci.yml)

# Compilation Guide

Compilation of Sigroute should be fairly simple. Firstly, make sure you have the following dependencies installed:
- libgtk-4-dev
- libpango1.0-dev
- libadwaita-1-dev
- libsqlite3-dev
- pkg-config
- cargo

Next, to compile and run the GUI, run "scripts/run-gui.sh". This will run the GUI in a development environment, and ensure the daemon is fully up to date and running (scripts/run-daemon.sh) before doing so.

# Dependencies & Licensing

The code in this repository is licensed under the AGPLv3:

> Sigroute
> Copyright (C) 2026 Sebastian Crookes
> 
> This program is free software: you can redistribute it and/or modify
> it under the terms of the GNU Affero General Public License as published
> by the Free Software Foundation, either version 3 of the License, or
> (at your option) any later version.
> 
> This program is distributed in the hope that it will be useful,
> but WITHOUT ANY WARRANTY; without even the implied warranty of
> MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
> GNU Affero General Public License for more details.
> 
> You should have received a copy of the GNU Affero General Public License
> along with this program.  If not, see <https://www.gnu.org/licenses/>.

The GUI application is built using [gtk4-rs](https://gtk-rs.org/), which provides the Rust bindings to GTK 4. The Rust bindings are utilised under the terms of the MIT license.

The GUI application also uses [libadwaita-rs](https://crates.io/crates/libadwaita), which provides the Rust bindings for libadwaita. Similarly, these bindings are utilised under the terms of the MIT license. The GUI application utilises [async-channel](https://github.com/smol-rs/async-channel) which is used under the terms of the MIT license. The GUI application utilises [serde-json](https://github.com/serde-rs/json) which is used under the terms of the MIT license. Finally, the GUI application utilises [chrono](https://github.com/chronotope/chrono) which is used under the terms of the MIT license.

The daemon uses [rusqlite](https://github.com/rusqlite/rusqlite) and [zvariant](https://github.com/z-galaxy/zbus/tree/main/zvariant), which are both utilised under the terms of the MIT license. It also uses [serde](https://github.com/serde-rs/serde), which is also utilised under the terms of the MIT license. 

Both applications (the GUI and the daemon) use [zbus](https://github.com/z-galaxy/zbus) and [tokio](https://github.com/tokio-rs/tokio), which are both utilised under the terms of the MIT license.
