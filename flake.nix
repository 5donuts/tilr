# tilr - A program to build an image from a set of image 'tiles'.
# Copyright (C) 2024  Charles German <5donuts@pm.me>
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.
{
  description = "Build an image from a set of image 'tiles'";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, ... }:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};

    rustOverrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
  in {
    # For details, see: https://nixos.wiki/wiki/Rust#Installation_via_rustup
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        clang
        llvmPackages_latest.bintools
        rustup
      ];

      RUSTC_VERSION = rustOverrides.toolchain.channel;
    };
  };
}
