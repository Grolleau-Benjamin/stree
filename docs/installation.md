# 🧩 Installation Guide

Arbor can be installed in multiple ways, depending on your system and workflow.

> [!WARNING]
> All of the following installation methods are related to `v1.0.0-alpha`. Please adjust the version number based on the latest release available.

## 🌍 One-liner (auto-detects your OS and architecture — Linux/Mac)

```bash
curl -fsSLO https://raw.githubusercontent.com/Grolleau-Benjamin/arbor/v1.0.0-alpha/install.sh
curl -fsSLO https://raw.githubusercontent.com/Grolleau-Benjamin/arbor/v1.0.0-alpha/install.sh.sha256
sha256sum -c install.sh.sha256 && bash install.sh
```

This command automatically detects your platform and installs the correct prebuilt binary in `/usr/local/bin`. It also completes the MAN and fish completions (if applicable).

Check that the installation was successful by running:

```bash
arbor --version
man arbor
```

## 🦀 Using Cargo (build from source)

If you already have Rust and Cargo installed:

```bash
cargo install --git https://github.com/Grolleau-Benjamin/arbor --tag v1.0.0-alpha --locked
```

or manually:

```bash
git clone --branch v1.0.0-alpha https://github.com/Grolleau-Benjamin/arbor.git
cd arbor
cargo build --release
sudo cp ./target/release/arbor /usr/local/bin
```

> [!TIP]
> This method don't install the manual and shell completions. You can find them in the `docs/` directory of the repository.

## 📦 Manual Installation (prebuilt archive)

1. Visit the [Release page](https://github.com/Grolleau-Benjamin/arbor/releases).
2. Download the archive corresponding to your system (e.g. `arbor-v1.0.0-alpha-x86_64-apple-darwin.tar.gz`).
3. Extract it and move the binary somewhere in your `PATH`, for example:
    ```bash
    sudo mv arbor /usr/local/bin && source ~/.zprofile
    ```
4. Then verify it works:
    ```bash
    arbor --version
    ```

## 🧹 Uninstallation
To uninstall Arbor, simply remove the binary from your system:

```bash
sudo rm /usr/local/bin/arbor
```

or use the script with the `--uninstall` flag:

```bash
bash install.sh --uninstall
```
This removes the binary, man page, and completions (if installed).
