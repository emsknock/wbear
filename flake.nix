{
	description = "A wee bear who has something to say";

	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
		flake-parts.url = "github:hercules-ci/flake-parts";
		fenix.url = "github:nix-community/fenix";

		flake-parts.inputs.nixpkgs-lib.follows = "nixpkgs";
		fenix.inputs.nixpkgs.follows = "nixpkgs";
	};

	outputs = inputs:
		inputs.flake-parts.lib.mkFlake {inherit inputs;} {
			systems = [
				"x86_64-linux"
				"x86_64-darwin"
				"aarch64-linux"
				"aarch64-darwin"
			];

			perSystem = {
				pkgs,
				shuf,
				inputs',
				...
			}: let
				toolchain = inputs'.fenix.packages.complete.toolchain;
				platform =
					pkgs.makeRustPlatform {
						cargo = toolchain;
						rustc = toolchain;
					};
				package =
					platform.buildRustPackage {
						pname = "wbear";
						version = "1.0.0";
						src = ./.;
						cargoLock.lockFile = ./Cargo.lock;
					};
			in {
				devShells.default = pkgs.mkShell {packages = [toolchain];};
				packages.default = package;
				packages.wbear = package;
			};
		};
}
