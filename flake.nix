{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    utils,
    naersk,
    ...
  }:
    utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
        naersk-lib = pkgs.callPackage naersk {};
        containerdSrc = builtins.fetchGit {
          url = "https://github.com/stratumix/rust-extensions";
          rev = "dbb1eb93578c9e1fffe06e8e482f94e6f1bfb757";
          submodules = true;
        };
        src = pkgs.runCommand "orvyn-src" {} ''
          mkdir -p "$out"
          cp -R ${./.}/. "$out/"
          rm -rf "$out/vendor/containerd-client"
          mkdir -p "$out/vendor/containerd-client"
          cp -R ${containerdSrc}/. "$out/vendor/containerd-client/"
        '';
      in {
        defaultPackage = naersk-lib.buildPackage {
          inherit src;
          nativeBuildInputs = [pkgs.protobuf];
        };
        devShell = with pkgs;
          mkShell {
            buildInputs = [protobuf cargo rustc rustfmt pre-commit rustPackages.clippy];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
      }
    );
}
