self: super:

let

  inherit (import ./lib.nix) fromJSONFile;

  pinned = (super.rustChannelOf (fromJSONFile ./rust-channel.json)).rust;

in

{

  rust = {
    cargo = pinned;
    rustc = pinned;
  };

}
