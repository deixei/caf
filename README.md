# CAF

Deixei Azure Cloud Adoption Framework CLI

## Depend on dx cli

Go to https://github.com/deixei/dx to know more, configure you ubuntu image


## Notes

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Learn

https://www.rust-lang.org/learn

### Start

```bash
cargo new caf
```

### dependencies

```bash
cargo add clap
cargo add colored
cargo add serde_yaml
```

### run locally

the base command is "cargo run" then the -- allows us to pass in all the cli options and parameters

```bash
cargo run -- run -n playbook -p ./playbooks/workspace2 -a STAGE=dev
cargo run -- help

cargo run -- -V
cargo run -- --version
```

## Author

[Marcio Parente](https://github.com/deixei) from deixei.com

To understand the overall context of this project read this book: [ENTERPRISE SOFTWARE DELIVERY: A ROADMAP FOR THE FUTURE](https://www.amazon.de/-/en/Marcio-Parente/dp/B0CXTJZJ2X/)
