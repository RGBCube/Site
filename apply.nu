#!/usr/bin/env nu

def --wrapped sync [...arguments] {
  rsync --rsh "ssh -q" --delete-missing-args --recursive --compress ...$arguments
}

# Applies the changes to the site by uploading it to the VPS.
def main [] {
  if (pwd | str starts-with "/data/data/com.termux") {
    sync ./ nine:site

    ssh -qtt nine "
      cd site
      LUME_DRAFTS=false nix run default#deno -- task build --location https://rgbcu.be/
    "

    sync nine:site/_site ./
  } else {
    LUME_DRAFTS=false deno task build --location https://rgbcu.be/
  }

  cd _site
  [cube disk] | par-each { sync ./ ($in + ":/var/www/site") }
  cd -

  echo $"(ansi green)Successfully uploaded!(ansi reset)"
}
