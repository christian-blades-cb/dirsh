{ sources ? import ./sources.nix }:

let
  pkgs = import sources.nixpkgs {
    overlays = [ (import sources.nixpkgs-mozilla) ];
  };
  chan = pkgs.rustChannelOfTargets "nightly" "2020-07-01" [ ];
in chan
