{sources ? import ./sources.nix}:

let
  pkgs = import sources.nixpkgs { overlays = [(import sources.nixpkgs-mozilla)]; };
  channel = "nightly";
  date = "2020-03-08";
  targets = [];
  chan = (pkgs.rustChannelOfTargets channel date targets).override {
    extensions = [
      "rust-src"
      "rls-preview"
      "clippy-preview"
      "rustfmt-preview"
    ];
  };
  #rustStableChannel = pkgs.latest.rustChannels.stable.rust.override {
    #extensions = [
      #"rust-src"
      #"rls-preview"
      #"clippy-preview"
      #"rustfmt-preview"
    #];
  #};
in
{
  rust = chan;
  rls = pkgs.rls;
  rustup = pkgs.rustup;
}
