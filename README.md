<div align="center">

  <h1><code>cargo-blinc</code></h1>

  <p>
    <strong>Instant feedback on the state of your tests<a href="#star">*<a/></strong>
  </p>
  <p>
  </p>

  <p>
    <a href="https://crates.io/crates/cargo-blinc"><img src="https://img.shields.io/crates/v/cargo-blinc" alt="Crates.io version" /></a>
    <a href="https://codecov.io/gh/devzbysiu/cargo-blinc">
  <img src="https://codecov.io/gh/devzbysiu/cargo-blinc/branch/master/graph/badge.svg?token=ELme4pPy8K" />
</a>
<a href="https://crates.io/crates/cargo-blinc">
  <img src="https://img.shields.io/crates/l/cargo-blinc" />
</a>
    <a href="https://docs.rs/cargo-blinc"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  </p>

  <h4>
    <a href="#about">About</a>
    <span> | </span>
    <a href="#demo">Demo</a>
    <span> | </span>
    <a href="#installation">Installation</a>
    <span> | </span>
    <a href="#configuration">Configuration</a>
  </h3>

  <sub>Built with 🦀</sub>
</div>

# <p id="about">About</p>

This crate allows to run arbitrary command and indicate the status of its execution using USB notification light - [blink(1)](https://blink1.thingm.com/).

<p id="star">*By default it runs <code>cargo test</code>. You can customize the command and LED colors using <a href="#configuration">configuration file</a>.

##### Example use case:

- run `cargo watch -x blinc`
- after every file save, it will start blinking with blue light
- it will start execution of `cargo test`
- after tests finish it will glow red when tests fail or green when tests succeed
</p>


# <p id="demo">Demo</p>

## --- TODO ---

# <p id="installation">Installation</p>
Run `cargo install cargo-blinc`

# <p id="configuration">Configuration</p>

By default no configuration is required if these settings satisfy you:
- task command: `cargo test`
- pending task color: **blue** (blinking)
- failed task color: **red**
- successful task color: **green**

You can control all of these by configuration file.

Run `cargo blinc --init` to initialize config. It will create **.blinc** file (note the dot) with following content:

```toml
[task]
cmd = "cargo"
args = ["test"]

[colors]
pending = ["blue", "white"]
failure = "red"
success = "green"
```
