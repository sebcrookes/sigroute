# Sigroute

I will explain what Sigroute is here, and add a lot more to this README file, when it has some basic features.

# Compilation Guide

Compilation of Sigroute should be fairly simple. Firstly, make sure you have the following dependencies installed:
- libgtk-4-dev
- libpango1.0-dev
- libadwaita-1-dev
- pkg-config
- cargo

Next, to compile and run the GUI, run "rungui.sh". This will run the GUI in a development environment, and ensure the daemon is fully up to date before doing so.

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

The GUI application also utilises [libadwaita-rs](https://crates.io/crates/libadwaita), which provides the Rust bindings for libadwaita. Similarly, these bindings are utilised under the terms of the MIT license.
