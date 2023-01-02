{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }: {
    defaultPackage.x86_64-darwin =
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit overlays;
          system = "x86_64-darwin";
        };
        rust-nightly = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
          extensions = [ "rust-src" ];
        });
      in
      with pkgs;
      stdenv.mkDerivation {
        name = "advent-of-code";
        src = self;
        buildInputs = [ rust-nightly python310 ];
      };
  };
}
