# rust-fire

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

## Usage

```bash
cargo run --release
```

You may resize the window.

Keys:

- `Alt-Enter`: Toggle maximized window
- `Esc`: Exit
