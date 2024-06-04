{ pkgs, mdbook-theme }:
let
	inherit (pkgs) stdenv;
	
	mdbook-theme-pkg = pkgs.rustPlatform.buildRustPackage {
		pname = "mdbook-theme";
		version = "0.1.4";
		cargoLock.lockFile = mdbook-theme.outPath + "/Cargo.lock";
		src = mdbook-theme;
	};

	tex = pkgs.texlive.combine {
		inherit (pkgs.texlive) scheme-small standalone qtree pict2e preview;
	};

	buildInputs = with pkgs; [
		wget git bash
		mdbook mdbook-admonish mdbook-plantuml
		mdbook-graphviz mdbook-theme-pkg mdbook-linkcheck
		plantuml graphviz tex poppler_utils];
	book = stdenv.mkDerivation {
		name = "rustemo-book";
		src = ../.;
		inherit buildInputs;

		buildPhase = ''
			cd docs
			./build-latex-images.sh
			mdbook build
		'';
		installPhase = ''
			mv book $out
		'';
	};
in
{
	inherit buildInputs;
	packages = { inherit book; };
}
