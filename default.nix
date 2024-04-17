{ rustPlatform
, SDL2
, SDL2_ttf
, SDL2_image
, pkg-config }:

rustPlatform.buildRustPackage rec {
  pname = "glungus";
  src = ./.;
  version = "0.1";
  cargoLock = {
    lockFile = ./Cargo.lock;
  };
  buildInputs = [
    SDL2
    SDL2_ttf
    SDL2_image
  ];
  nativeBuildInputs = [
    pkg-config
  ];
}
