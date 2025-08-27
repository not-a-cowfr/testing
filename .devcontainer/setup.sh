apt-get update
apt-get install -y \
  nushell \

curl https://sh.rustup.rs -sSf | sh -s -- -y 
rustup install nightly
rustup component add rustfmt
rustup component add rustfmt --toolchain nightly
rustup component add clippy 
rustup component add clippy --toolchain nightly

echo "\$env.config.buffer_editor = \"code\"" > ~/.config/nushell/env.nu
echo "\$env.config.show_banner = false" >> ~/.config/nushell/env.nu