# AA Client

```bash
PB_REL="https://github.com/protocolbuffers/protobuf/releases"
curl -LO $PB_REL/download/v25.1/protoc-25.1-linux-x86_64.zip
unzip protoc-25.1-linux-x86_64.zip -d $HOME/.local
ln -s /root/.local/bin/protoc /usr/bin/protoc

yum install -y clang
RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

cargo build --release

# Do attestation
./target/release/aa-client -a http://127.0.0.1:50002 attestation --token-type coco_as
```