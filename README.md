# TPS Cube

This is an open source application for tracking cube solves. This is the code that runs on [tpscube.xyz](https://tpscube.xyz).

It is written in pure Rust using the [egui](https://github.com/emilk/egui) framework.

* Automatic scramble generation.
* Streamlined session tracking optimized for quick practice sessions.
* Solve history that isn't lost when quick resetting sessions.
* Bluetooth cube support with split timing and advanced stats.
* Graphical reports for tracking progress.
* Automatic cloud sync across all your devices without any privacy concerns. Your data is completely anonymous, is shared only when you explicitly share solves, and there are no ads of any kind.
* (Planned) Algorithm library and practice mode.

# Supported platforms

* Anything that runs a web browser with WebGL support
* Windows / MacOS / Linux with a native binary (and without Electron!)

# Scrambling algortihms

Popular scrambling algorithms, including the official WCA scramble, are licensed under GPLv3. Normally
this would not be an issue, as this project is fully open source. Unfortunately, the GPLv3 license is
incompatible with the iOS App Store, even if the application is entirely open source. Because of this,
the scrambling algorithm implementations in this program are new and licensed under the MIT license.
These algorithms are free to use anywhere (even in commercial products) and are fully compatible with
all app store licensing restrictions.

Please note that the scrambles are _not_ competition legal. Use
[tnoodle](https://www.worldcubeassociation.org/regulations/scrambles/) for competitions.
