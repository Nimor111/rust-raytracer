{ sources ? import ./nix/sources.nix, pkgs ? import sources.nixpkgs {} }:

let
  rustSrc = import ./nix/rust.nix { inherit sources; };

  naersk = pkgs.callPackage sources.naersk {
    rustc = rustSrc.rust;
    cargo = rustSrc.rust;
  };

  src = builtins.filterSource
    (path: type: type != "directory" || builtins.baseNameOf path != "target")
    ./.;
in naersk.buildPackage {
  inherit src;
  nativeBuildInputs = [pkgs.csfml];
  remapPathPrefix =
    true; # remove nix store references for a smaller output package
}
