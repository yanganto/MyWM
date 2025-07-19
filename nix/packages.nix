{ pkgs, self, crane }:
let
  craneLib = crane.mkLib pkgs;
  cargoToml = "${self}/Cargo.toml";
  cargoTomlConfig = builtins.fromTOML (builtins.readFile cargoToml);
  version = cargoTomlConfig.package.version;
  src = self;
  buildInputs = with pkgs; [ 
      xorg.libX11
      xorg.libXft
      pkg-config
    ];
  nativeBuildInputs = with pkgs; [ pkg-config ];
  doCheck = false;
in
rec {
  default = mwm;

  mwm = craneLib.buildPackage {
    inherit version src cargoToml nativeBuildInputs doCheck;
    inherit buildInputs;
    pname = "mwm";
    cargoArtifacts = craneLib.buildDepsOnly {
      inherit version src cargoToml nativeBuildInputs doCheck;
      pname = "mwm";
      inherit buildInputs;
    };
  };
}
