with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "rssnek";

  buildInputs = [ rustc cargo ncurses ];
}

