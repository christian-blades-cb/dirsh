{ sources ? import ./sources.nix }:

let
  pkgs = import sources.nixpkgs {
    overlays = [ (import sources.nixpkgs-mozilla) ];
  };
  chan = pkgs.latest.rustChannels.nightly;
in chan
