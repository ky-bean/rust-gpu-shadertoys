{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs =
    {
      self,
      nixpkgs,
      ...
    }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          pkg-config
          rustup
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          xorg.libxcb
          libGL
          vulkan-loader
          wayland
          libxkbcommon
          spirv-tools
          glibc
        ];

        # Runtime dependencies.
        LD_LIBRARY_PATH =
          with pkgs;
          lib.makeLibraryPath [
            xorg.libX11
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            xorg.libxcb
            libGL
            vulkan-loader
            wayland
            libxkbcommon
            glibc
          ];

        nativeBuildInputs = [ pkgs.pkg-config ];
      };
    };
}
