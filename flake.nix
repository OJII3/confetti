{
  description = "Confetti - Screen-wide confetti animation for Linux";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-linux" ];

      perSystem = { config, self', inputs', pkgs, system, ... }:
        let
          rustToolchain = pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" "rust-analyzer" ];
          };
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ inputs.rust-overlay.overlays.default ];
          };

          packages = {
            # Rust + GTK4 version
            rust = pkgs.rustPlatform.buildRustPackage {
              pname = "confetti-rust";
              version = "0.1.0";
              src = ./.;
              cargoLock.lockFile = ./Cargo.lock;

              nativeBuildInputs = with pkgs; [
                pkg-config
                wrapGAppsHook4
              ];

              buildInputs = with pkgs; [
                gtk4
                gtk4-layer-shell
                glib
                cairo
                pango
                gdk-pixbuf
                graphene
              ];
            };

            # GNOME Shell Extension version
            gnome-extension = pkgs.stdenv.mkDerivation {
              pname = "confetti-gnome-extension";
              version = "0.1.0";
              src = ./gnome-extension;

              dontBuild = true;

              installPhase = ''
                runHook preInstall

                # Install extension files
                extensionDir=$out/share/gnome-shell/extensions/confetti@ojii3.github.com
                mkdir -p $extensionDir
                cp extension.js metadata.json $extensionDir/

                # Install CLI trigger
                mkdir -p $out/bin
                cp confetti $out/bin/confetti
                chmod +x $out/bin/confetti

                runHook postInstall
              '';

              passthru = {
                extensionUuid = "confetti@ojii3.github.com";
              };

              meta = with pkgs.lib; {
                description = "GNOME Shell Extension for confetti animation";
                homepage = "https://github.com/ojii3/confetti";
                license = licenses.mit;
                maintainers = [ ];
              };
            };

            # Default: GNOME Extension (recommended)
            default = self'.packages.gnome-extension;
          };

          devShells.default = pkgs.mkShell {
            inputsFrom = [ ];

            nativeBuildInputs = with pkgs; [
              rustToolchain
              pkg-config
              cargo-watch
            ];

            buildInputs = with pkgs; [
              gtk4
              gtk4-layer-shell
              glib
              cairo
              pango
              gdk-pixbuf
              graphene
            ];

            RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          };
        };
    };
}
