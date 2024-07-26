# CAF

Deixei Azure Cloud Adoption Framework CLI




## Notes

### Install Rust

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

### Learn

https://www.rust-lang.org/learn

### Start

cargo new caf

### dependencies

cargo add clap
cargo add serde_yaml

### run locally

the base command is "cargo run" then the -- allows us to pass in all the cli options and parameters

cargo run -- run -n playbook -p ./playbooks/workspace2 -a STAGE=dev


## Author

[Marcio Parente](https://github.com/deixei) from deixei.com

To understand the overall context of this project read this book: [ENTERPRISE SOFTWARE DELIVERY: A ROADMAP FOR THE FUTURE](https://www.amazon.de/-/en/Marcio-Parente/dp/B0CXTJZJ2X/)
