{ pkgs }:
{
  default = pkgs.mkShell ({
    buildInputs = with pkgs; [ 
      rust-bin.stable."1.87.0".minimal
      xorg.libX11
      xorg.libXft
      pkg-config
    ];
  });
}
