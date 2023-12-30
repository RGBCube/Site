{
  description = "The official website and link portal of RGBCube and his work.";

  nixConfig = {
    extra-substituters        = "https://cache.garnix.io/";
    extra-trusted-public-keys = "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g=";
  };

  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-unstable";
    };

    flake-utils = {
      url = "github:numtide/flake-utils";
    };

    cargo2nix = {
      url                        = "github:cargo2nix/cargo2nix";
      inputs.nixpkgs.follows     = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = { nixpkgs, flake-utils, cargo2nix, ... } @ inputs: flake-utils.lib.eachDefaultSystem (system: let
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ cargo2nix.overlays.default ];
    };

    rustPackages = pkgs.rustBuilder.makePackageSet {
      rustVersion = "1.76.0";
      rustChannel = "nightly";
      rustProfile = "minimal";
      packageFun  = import ./Cargo.nix;
    };
  in rec {
    devShells.default = rustPackages.workspaceShell {
      packages = [ cargo2nix.packages.${system}.default ];
    };

    packages.site    = rustPackages.workspace.site {};
    packages.default = packages.site;
  });
}
