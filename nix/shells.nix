{ pkgs }:
{
  default = pkgs.mkShell ({
    buildInputs = with pkgs; [ 
      rust-bin.stable.latest.minimal
      xorg.libX11
      xorg.libXft
      pkg-config
    ];
  });
}
