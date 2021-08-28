{ pkgs ? import <nixpkgs> { }
, lib ? pkgs.lib
, rustPlatform ? pkgs.rustPlatform
}:

rustPlatform.buildRustPackage rec {
  pname = "earmms-keyderiv";
  version = "2.0.0";
  src = ./.;

  cargoDepsName = pname;
  cargoSha256 = "1mg9ylavy585zynrxx6153n0pfpbdwq3zgk0j9bd6miibhj6zq4y";

  buildInputs = with pkgs; [
    pkg-config
    openssl
    libsodium
  ];

  RUST_SODIUM_USE_PKG_CONFIG = 1;

  meta = with lib; {
    description = "Key derivation microservice for EARMMS and other PAPA projects";
    homepage = "https://gitlab.com/againstprisons/earmms_keyderiv";
    platforms = platforms.all;
    license = licenses.mit;
    maintainers = [ ];
  };
}
