# AA Client

Update Kernel
```bash
yum update kernel -y
```

Then reboot.

Install Dependencies
```bash
# Protobuf
yum install -y clang unzip git
PB_REL="https://github.com/protocolbuffers/protobuf/releases"
curl -LO $PB_REL/download/v25.1/protoc-25.1-linux-x86_64.zip
unzip protoc-25.1-linux-x86_64.zip -d $HOME/.local
ln -s /root/.local/bin/protoc /usr/bin/protoc

# Rust
export RUST_DIST_SERVER="https://rsproxy.cn"
export RUST_UPDATE_ROOT="https://rsproxy.cn/rustup"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
### Press Enter to continue

cat << EOF > ~/.cargo/config
[source.crates-io]
replace-with = 'aliyun' # 指定使用下面哪个源，修改为source.后面的内容即可
#阿里云
[source.aliyun]
registry = "sparse+https://mirrors.aliyun.com/crates.io-index/"
EOF

# DCAP stack
region="cn-beijing"

yum install -y yum-utils
yum-config-manager --add-repo https://enclave-${region}.oss-${region}-internal.aliyuncs.com/repo/alinux/enclave-expr.repo
yum install -y libtdx-attest-devel
```

Launch AA
```bash
git clone https://github.jobcher.com/gh/https://github.com/Xynnn007/guest-components.git
cd guest-components/attestation-agent && git reset --hard 7efbe34c5b7f225c0308078343af967ef68c2945 && make ttrpc=false ATTESTER=tdx-attester
cd ..

TRUSTEE_ADDR=xxx

tee << EOF > config.toml
[token_configs]

[token_configs.coco_as]
url = "${TRUSTEE_ADDR}/as"

[token_configs.kbs]
url = "${TRUSTEE_ADDR}"

[eventlog_config]

eventlog_algorithm = "sha384"
init_pcr = 17
enable_eventlog = true
EOF

./target/x86_64-unknown-linux-gnu/release/attestation-agent -c config.toml
```


Then use this tool
```bash
cargo build --release

# Do attestation
./target/release/aa-client -a http://127.0.0.1:50002 attestation --token-type coco_as

# Record App Eventlog
./target/release/aa-client -a http://127.0.0.1:50002 record-app-eventlog \
    --domain aliyun \
    --operation loadContents \
    --content sha256=12345

# Do attestation
./target/release/aa-client -a http://127.0.0.1:50002 attestation --token-type coco_as

# Then you can view the log on trustee log side,
# or base64 decode the second part of the returned token splited by `.`
# You can see a JSON object whose `tdx.aael.aliyun/loadContents` is `["sha256=12345"]`
```