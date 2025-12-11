{
  description = "my project description";

  outputs = { self, nixpkgs }:
  {
    defaultPackage.x86_64-linux =
      let
        pkgs = nixpkgs.legacyPackages.x86_64-linux;
      in
        pkgs.rustPlatform.buildRustPackage (finalAttrs: {
          pname = "cardauth";
          version = "0.1.0";

          src = self;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        });
  };
}

