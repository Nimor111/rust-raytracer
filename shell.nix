{sources ? import ./nix/sources.nix}:

let
  overlay = _: pkgs: { niv = import sources.niv {}; };
  rustSrc = import ./nix/rust.nix { inherit sources; };
  pkgs = import sources.nixpkgs { overlays = [overlay]; };
  nivSrc = pkgs.niv;
in
pkgs.mkShell {
  buildInputs = [
    nivSrc.niv
    rustSrc.rust
    rustSrc.rls
    rustSrc.rustup
  ];
}
