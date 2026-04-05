# External / Reference Materials

Place **third-party** or **instructor-provided** reference material here with provenance notes. Keep student-authored scripts in `../scripts/`.

## Reference Resources Mentioned in Course

### Official Rust Documentation

- **The Rust Programming Language** (official book) — <https://doc.rust-lang.org/book/>
  - Chapter 2: *Programming a Guessing Game* (used as Week 5 tutorial)
  - Chapter 4: *Understanding Ownership*
  - Chapter 5: *Using Structs to Structure Related Data*
- **Rust by Example** — <https://doc.rust-lang.org/rust-by-example/>
- **Cargo Documentation** — <https://doc.rust-lang.org/cargo/>
- **The `rand` Crate** — <https://docs.rs/rand/>

### Instructor-Shared Links (Week 1 Lecture Chat, 2025-01-08)

> [!NOTE]
> The following links were shared by Instructor Travis Czech during the Week 1 live lecture via Zoom chat.

| Resource | URL | Context |
|---|---|---|
| Rust Programming Course (YouTube playlist) | <https://www.youtube.com/watch?v=4Dj09n5T67s&list=PLaYMx1eoXk4Lt8C2R8vFybJUy5E7OlEoh&index=12> | Supplementary video series on Rust |
| Visual Studio Code | <https://code.visualstudio.com/> | Recommended editor for the course |
| VS Code for Linux (Debian package) | <https://code.visualstudio.com/docs/?dv=linux64_deb> | Direct download for VM setup |
| Rustup installer | <https://rustup.rs/> | Official Rust toolchain installer |

### Installation Commands (from Week 1 Chat)

```bash
# VS Code installation on Debian/Kali
sudo dpkg -i code_1.96.2-1734607745_amd64.deb
# or
sudo apt install code

# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# First Cargo project
cargo new wk1
cargo run
```

## Provenance Notes

All referenced external resources are publicly available official documentation. No third-party proprietary code is stored in this folder. Chat links were extracted from the Week 1 lecture chat log (stored at `D:\CC\...\Week 1\Links in the chat - CSEC Tool Development - Travis Czech.docx`).
