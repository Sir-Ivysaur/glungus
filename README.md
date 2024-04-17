# glungus
a stupid clicker made in rust and SDL2 to ensure *blazingly* fast performance so you can enjoy clicking glungus!

expect really terrible code practice thsi is just a shitpost

# installation / running

## Without nix

* Install `libSDL2`, `libSDL2_ttf` and `libSDL2_image` on your system

* Run `cargo build --release`

* _(optional)_ Move the executable from `./target/release/glungus` somewhere in `$PATH`

* ***G L U N G U S***

## With nix

* To build it: `nix build .#glungus` (the executable will be `./result/bin/glungus`)

* To run it: `nix run .#glungus`

### 
![glungus](/assets/glungus.png)
look he's memory safe!! wow

---
made with love by niko & friends!
