{ naersk, src, lib, pkg-config, openssl, cmake, postgresql }:

naersk.buildPackage {
  pname = "telegram-decoder";
  version = "0.1.0";

  src = ./.;

  cargoSha256 = lib.fakeSha256;

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl cmake postgresql ];

  meta = with lib; {
    description = "Sever which receives raw data and turns it into telegrams";
    homepage = "https://github.com/dump-dvb/telegram-decoder";
  };
}
