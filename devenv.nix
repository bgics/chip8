{ pkgs, ... }:
{
  languages.rust.enable = true;

  packages = with pkgs; [ taplo ];
}
