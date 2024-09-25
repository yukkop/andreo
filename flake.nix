{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          rustToolchain = (pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);
	  nativeBuildInputs = with pkgs; [ rustToolchain pkg-config ];
          buildInputs = with pkgs; [ 
	    openssl 
	    alsa-lib

            vulkan-loader

	    xorg.libX11
	    xorg.libXrandr
	    xorg.libXcursor
	    xorg.libXi

	    wayland
	    libxkbcommon

	    udev
	    shellcheck
	    file
	  ];
        in
        with pkgs;
        {
          devShells.default = mkShell {
            inherit buildInputs nativeBuildInputs;
	    LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
          };
        }
      );

}
