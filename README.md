# rust-fire

![fire-1 0 1](https://user-images.githubusercontent.com/67230/54908538-6aa3a700-4ee8-11e9-8dcc-1304767e0226.gif)

Reviving an old childhood memory, where I implemented a fire in
`mov ax,13h; int 10h` graphics mode (320x200 at 8 bit indexed colors).

A fun way to attempting to learn Rust!

## Prerequisites

### Cargo and Rust

https://www.rust-lang.org/tools/install

### Libraries

- sdl2
- sdl2-gfx

On Ubuntu, you can install the libraries with:

```bash
sudo apt install -y libsdl2-dev libsdl2-gfx-dev
```

On Fedora, install the libraries with:

```bash
sudo dnf install -y SDL2-devel SDL2_gfx-devel
```

## Usage

```bash
cargo run --release
```

You may resize the window.

Keys:

- `Alt-Enter`: Toggle maximized window
- `Esc`: Exit
