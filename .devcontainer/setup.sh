curl -fsSL https://apt.fury.io/nushell/gpg.key | sudo gpg --dearmor -o /etc/apt/trusted.gpg.d/fury-nushell.gpg
echo "deb https://apt.fury.io/nushell/ /" | sudo tee /etc/apt/sources.list.d/fury.list
sudo apt update
sudo apt install -y \
  nushell \

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
bash
rustup install nightly
rustup component add rustfmt
rustup component add rustfmt --toolchain nightly
rustup component add clippy 
rustup component add clippy --toolchain nightly

touch ~/.config/nushell/env.nu
echo "\$env.config.buffer_editor = \"code\"" > ~/.config/nushell/env.nu
echo "\$env.config.show_banner = false" >> ~/.config/nushell/env.nu