{ pkgs }:

pkgs.mkShell {
  packages = with pkgs; [ cargo SDL2 SDL2_image SDL2_ttf ];
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
