# ALT Linux packages differentiator

### How to build project

```
su -
apt-get update
apt-get dist-upgrade

apt-get install clippy rust rust-cargo rust-src libcrypto3 libssl3 libssl-devel openssl

cargo build --release
```

### How to run CLI

```
cargo run -- --repo-1 <repo-1> --repo-2 <repo-2>
```

### Packages

I have a system installed on ALT Linux for a long time, so I can skip some packages that are required to run this project. 
You will probably need to install additional LLVM auxiliary libraries.

### License

MIT

### Author

[Kuat Kurpenov](https://github.com/kurpenok "Kuat Kurpenov")
