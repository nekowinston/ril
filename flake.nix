{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = {
    self,
    fenix,
    nixpkgs,
  }: let
    systems = ["aarch64-darwin" "aarch64-linux" "x86_64-darwin" "x86_64-linux"];
    overlays = [fenix.overlays.default];
    forEachSystem = fn: lib.genAttrs systems (system: fn (import nixpkgs {inherit overlays system;}));
    inherit (nixpkgs) lib;
  in {
    devShells = forEachSystem (pkgs: let
      toolchain = pkgs.fenix.fromToolchainFile {
        dir = ./.;
        sha256 = "sha256-Q9UgzzvxLi4x9aWUJTn+/5EXekC98ODRU1TwhUs9RnY=";
      };
    in {
      default = pkgs.mkShell {
        buildInputs = [toolchain pkgs.rust-analyzer];
        env.RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
      };
    });
  };
}
