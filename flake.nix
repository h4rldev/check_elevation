{
  description = "check_elevation - Check if the current process is elevated. A successor to the `is_elevated` crate.";

  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.zst";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }: let
    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];

    eachSystem = nixpkgs.lib.genAttrs systems;
  in {
    devShells = eachSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [rust-overlay.overlays.default];
      };

      # No prebuilt Windows std on purpose: the shell exists to cross-check
      # the crate as no_std. `-Z build-std=core` then compiles with only
      # core available, so any `std` use in the graph fails.
      rust = pkgs.rust-bin.nightly.latest.default.override {
        extensions = ["rust-src" "rustfmt" "clippy"];
      };
    in {
      default = pkgs.mkShell {
        packages = [rust];
      };
    });
  };
}
