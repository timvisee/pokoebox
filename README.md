[![Build status on GitLab CI][gitlab-ci-master-badge]][gitlab-ci-link]

[gitlab-ci-link]: https://gitlab.com/timvisee/pokoebox/pipelines
[gitlab-ci-master-badge]: https://gitlab.com/timvisee/pokoebox/badges/master/pipeline.svg

# PokoeBox
A custom speaker project.

This is still a work in progress. The README will be updated anytime soon.

![PokoeBox Player Screenshot](./res/pokoebox-player-screenshot.png)

## Requirements
* Linux
* Rust 1.40

Required libraries:
* `libgtk-3-dev`
    * Apt: `sudo apt install libgtk-3-dev`
* `at-spi2-core` (only with `rpi` feature)
    * Apt: `sudo apt install at-spi2-core`
* `libasound2-dev`:
    * Apt: `sudo apt install libasound2-dev`

## Build
### Compile features
The following compile time features are available:
* `rpi`: Compile with Raspberry Pi features, with support for GPIO and custom external peripherals.

Example: `cargo run --features "rpi"`

## License
This project is released under the GNU GPL-3.0 license.
Check out the [LICENSE](LICENSE) file for more information.
