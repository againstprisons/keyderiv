{ pkgs ? import <nixpkgs> { }
, lib ? pkgs.lib
, system ? builtins.currentSystem
}:

let
  containerPkgs = import <nixpkgs> {
    inherit system;
    overlays = [
      (import ./overlay.nix)
    ];
  };

  inherit (pkgs.dockerTools) buildImage;

in buildImage {
  name = "earmms-keyderiv";
  contents = containerPkgs.earmms-keyderiv;

  config = {
    Env = [ "PORT=80" "RUST_LOG=info"];
    Cmd = [ "${containerPkgs.earmms-keyderiv}/bin/earmms-keyderiv" ];
  };
}
