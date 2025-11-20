<h1 align="center">🌴 Arbor — The Smart Tree</h1>
<p align="center"><i>by Benjamin Grolleau & Angelo Tunney</i></p>

<p align="center">
  <img src="./img/logo.jpg" alt="Arbor Logo" width="180" style="border-radius: 24px; margin-top: 12px;" />
</p>

<p align="center">
  <a href="https://www.rust-lang.org">
    <img src="https://img.shields.io/badge/language-Rust-orange?logo=rust" alt="Rust">
  </a>
  <img src="https://img.shields.io/badge/status-in%20prod-green" alt="Status">
  <img src="https://img.shields.io/github/v/tag/Grolleau-Benjamin/arbor?label=version" alt="Version">
</p>

Arbor is a modern and smart reimplementation of the traditional `tree` command. Its goal is to provide developers with a more intuitive and visually clear way to explore project structures. Unlike the classic version, Arbor introduces several improvements such as configurable depth levels for better readability, colorized output and added icons to distinguish files and directories at a glance, and automatic filtering of files or folders ignored by Git (based on the .gitignore file). It can also display the Git status of files, showing whether they are untracked, modified, staged for commit, or stashed. Arbor is written in Rust.

![Screen](./img/screen.png)

## 🚀 Quick start

```bash
curl -fsSLO https://raw.githubusercontent.com/Grolleau-Benjamin/arbor/main/install.sh
curl -fsSLO https://raw.githubusercontent.com/Grolleau-Benjamin/arbor/main/install.sh.sha256
sha256sum -c install.sh.sha256 && bash install.sh
```

Then run:
```bash
arbor -d 2 -c always -i
# equivalent to
arbor --depth 2 --color always --icons
```

### 🔐 Security
The installation checksum file (`install.sh.sha256`) is GPG-signed, and the project owner’s public key is available for verification.
You can verify the signature using the following commands:
```
curl -fsSLO https://raw.githubusercontent.com/Grolleau-Benjamin/arbor/main/install.sh.sha256.sig
curl -fsSLO https://raw.githubusercontent.com/Grolleau-Benjamin/arbor/main/public-key.asc
gpg --import public-key.asc
gpg --verify install.sh.sha256.sig install.sh.sha256

# You should see :
# gpg: Good signature from "Benjamin Grolleau (Arbor) <...>"
```

## 📘 Documentation

- [Installation guide](./docs/installation.md)
- [Contributing guide](./CONTRIBUTING.md)
- [CLI usage and options](./docs/usage.md)
- [Comparison with Eza](./docs/comparison.md)

## 🤝 Contributing

Contributions, issues, and feature requests are welcome!
Feel free to open a [pull request](https://github.com/Grolleau-Benjamin/arbor/pulls).

## 👥 Contributors

<table align="center">
  <tr>
    <td align="center">
      <a href="https://github.com/Grolleau-Benjamin">
        <img src="https://images.weserv.nl/?url=avatars.githubusercontent.com/u/127044450?v=4&h=90&w=90&fit=cover&mask=circle" width="90" height="90" alt="Benjamin Grolleau"/><br/>
        <sub><b>Benjamin Grolleau</b></sub>
      </a>
    </td>
    <td align="center">
      <a href="https://github.com/angelo-tny">
        <img src="https://images.weserv.nl/?url=avatars.githubusercontent.com/u/116174527?v=4&h=90&w=90&fit=cover&mask=circle" width="90" height="90" alt="Angelo Tunney"/><br/>
        <sub><b>Angelo Tunney</b></sub>
      </a>
    </td>
  </tr>
</table>

## 📜 License
Licensed under the **Apache-2.0** License.
See [LICENSE](./LICENSE) for more information.
