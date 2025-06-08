{
  inputs = {
    utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, utils }: utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          gcc
    SDL2
    SDL2_ttf
    SDL2_mixer
    SDL2_image
    SDL2_gfx
          rust-analyzer
          fish
          clippy
          trunk
        ];

        shellHook = ''
          exec fish
        '';
      };
    }
  );
}
