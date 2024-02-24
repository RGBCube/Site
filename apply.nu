#!/usr/bin/env nu

# Applies the changes to the site by uploading it to the VPS.
def main [] {
  deno task build --location https://rgbcu.be/

  cd _site
  rsync --delete --recursive --compress ./ cube:/var/www/site
  cd -

  echo $"(ansi green)Successfully uploaded!(ansi reset)"
}
