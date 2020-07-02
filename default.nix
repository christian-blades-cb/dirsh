{ sources ? import ./nix/sources.nix, pkgs ? import sources.nixpkgs { } }:
let
  rust = import ./nix/rust.nix { inherit sources; };
  naersk = pkgs.callPackage sources.naersk {
    rustc = rust;
    cargo = rust;
  };
  filterTarget = path: type: type != "directory" || builtins.baseNameOf path != "target";
in naersk.buildPackage {
  src = with pkgs.lib; cleanSourceWith {
    src = cleanSource ./.;
    filter = filterTarget;
  };

  remapPathPrefix = true;
}
