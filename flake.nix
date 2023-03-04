{
  description = "This is a flake for building my pinyin-tone-marks Rust app";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShell = pkgs.mkShell
          {
            # The packages we need for this project
            buildInputs = with pkgs;
              [
                rustc
                cargo
                clippy
                libiconv
                rust-analyzer
                rustfmt
                exercism
              ];
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          };
      });
}
