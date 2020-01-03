<div align="center">

  <h1><code>cargo-blinc</code></h1>

  <p>
    <strong>Instant feedback on the state of your tests<a href="#star">*</a></strong>
  </p>

  <p>
    <a href="https://crates.io/crates/cargo-blinc">
      <img src="https://img.shields.io/crates/v/cargo-blinc?color=%2388C0D0&logoColor=%234C566A&style=flat-square" alt="Crates.io version" />
    </a>
    <a href="https://codecov.io/gh/devzbysiu/cargo-blinc">
      <img src="https://img.shields.io/codecov/c/github/devzbysiu/cargo-blinc?color=%2388C0D0&logoColor=%234C566A&style=flat-square&token=bfdc4b9d55534910ae48fba0b8e984d0" alt="Code coverage"/>
    </a>
    <a href="https://crates.io/crates/cargo-blinc">
      <img src="https://img.shields.io/crates/l/cargo-blinc?color=%2388C0D0&logoColor=%234C566A&style=flat-square" alt="License"/>
    </a>
    <a href="https://docs.rs/cargo-blinc">
      <img src="https://img.shields.io/badge/docs-latest-blue.svg?color=%2388C0D0&logoColor=%234C566A&style=flat-square" alt="docs.rs docs" />
    </a>
  </p>

  <h4>
    <a href="#about">About</a>
    <span> | </span>
    <a href="#demo">Demo</a>
    <span> | </span>
    <a href="#installation">Installation</a>
    <span> | </span>
    <a href="#configuration">Configuration</a>
    <span> | </span>
    <a href="#license">License</a>
    <span> | </span>
    <a href="#contribution">Contribution</a>
  </h3>

  <sub>Built with 🦀</sub>
</div>

# <p id="about">About</p>

This crate allows to run arbitrary commands and indicate the status of its execution using USB notification light - [blink(1)](https://blink1.thingm.com/).

<p id="star">*By default it runs <code>cargo test</code>. You can customize the commands and LED colors using <a href="#configuration">configuration file</a>.

##### Example use case:

- run `cargo watch -x blinc`
- after every file save, it will start blinking with blue light
- it will start execution of `cargo test`
- after tests finish it will glow red when tests fail or green when tests succeed
</p>


# <p id="demo">Demo</p>

## --- TODO ---

# <p id="installation">Installation</p>

To install
```
cargo install cargo-blinc
```

To upgrade
```
cargo install --force cargo-blinc
```

# <p id="configuration">Configuration</p>

By default no configuration is required if these settings satisfy you:
- tasks: `cargo check`, `cargo test`
- pending task color: **blue** (blinking)
- failed task color: **red**
- successful task color: **green**

You can control all of these by configuration file.

Run `cargo blinc --init` to initialize config. It will create **.blinc** file (note the dot) with following content:

```toml
[[tasks]]
cmd = "cargo"
args = ["check"]

[[tasks]]
cmd = "cargo"
args = ["test"]

[colors]
pending = ["blue", "white"]
failure = "red"
success = "green"
```

# <p id="license">License</p>

This project is licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

# <p id="contribution">Contribution</p>

## --- TODO ---

