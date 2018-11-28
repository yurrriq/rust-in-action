{

  fetchTarballFromGitHub =
    { owner ? "NixOS", repo ? "nixpkgs-channels"
    , rev ? "nixpkgs-18.03-darwin", sha256 ? null, ... }:
    builtins.fetchTarball {
     url = "https://github.com/${owner}/${repo}/tarball/${rev}";
     inherit sha256;
    };

  fromJSONFile = f: builtins.fromJSON (builtins.readFile f);

}
