{ pkgs ? import <nixpkgs> {} }:
with pkgs;
mkShell {
  nativeBuildInputs = [
    gcc
    SDL2
    SDL2_ttf
    SDL2_mixer
    SDL2_image
    SDL2_gfx
  ];
}
