# wbear

```
  It's pronounced "wee bear".
     \
    ʕ•ﻌ•ʔ
```

Just a small program that works like `fortune | cowsay` but with my own quote list and a bear instead of a cow. Oh, and running ~~blazingly fast~~ close to 50km/h at max speed; the quotes are baked in to the binary itself.

## Installing

This repo is a nix flake with package outputs, so you'd add the repo as an `input` and then install it with `systemPackages = [inputs.wbear.packages."x86_64-linux".default]` or e.g. call it in a script with `lib.getExe inputs.wbear.packages."x86_64-linux".default`. Or you can just try it out with e.g. `nix run github:emsknock/wbear`.

If you've somehow found this repo and want to run this on a non-nix system, woah, hello! Feel free to open an issue asking for a prebuilt binary.

## Configuring

This is such a minimal program I'd recommend you just fork it instead and edit the source to your liking.
