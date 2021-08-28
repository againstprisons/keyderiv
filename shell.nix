let
  pkgs = import <nixpkgs> {
    overlays = [
      (import ./overlay.nix)
    ];
  };

  inherit (pkgs) lib;

in pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    rustc
    rustfmt
    pkg-config
    openssl
    libsodium
  ];

  RUST_SODIUM_USE_PKG_CONFIG = 1;
}
