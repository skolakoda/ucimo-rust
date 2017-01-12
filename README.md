# Učimo Rust

[Rust](https://www.rust-lang.org/en-US/) je sistemski jezik koji se izvršava zapanjujuće brzo, predupređuje greške segmentacije i jemči bezbednost.

Učimo po zvaničnom tutorijalu: [The Rust Programming Language](https://doc.rust-lang.org/book/)

## Instalacija

Ako koristiš Linux ili Mac, otvori terminal i ukucaj ovo:

```
curl -sSf https://static.rust-lang.org/rustup.sh | sh
```

Ako je Rast uspešno instaliran, možeš proveriti verziju:
```
rustc --version
```

## Kompajliranje

Izvorne fajlove pišeš sa `.rs` ekstenzijom (npr. `main.rs`), a kompajliraš ih i pokrećeš na sledeći način:
```
rustc main.rs
./main
```

Rustc je očigledno skraćeno od Rust compiler :)
