---
title: Swap the ı and i key on your keyboard for faster modal editing
description: How to swap the ı and i key on your Turkish keyboard on Linux.

date: 2024-05-20

tags:
- localisation
- modal-editors
---

If you have ever used a Turkish-Q keyboard in combination with a modal
editor before, you might have noticed that the `i` key is pretty far off
to the side:

![The placement of the `i` key on the Turkish-Q layout](/assets/turkish-q-i.webp)

This blog post will guide you on how to swap the `ı` key with the `i` key
(but not the `I` and `İ` keys). This will be a great change if you write primarily
in English but need the Turkish symbols sometimes.

> There is `tr(us)` in the `xkeyboard_config` package that does something similar to this:
> 
> ```cpp
> // The basic Turkish layout with "i" and "ı" swapped.
> // Originally by Ali Riza KESKIN <parduscix@yandex.ru>, 2021.
> partial
> xkb_symbols "us" {
> 
>     include "tr(basic)"
> 
>     name[Group1]="Turkish (i and ı swapped)";
> 
>     key <AC11>  { type[group1] = "FOUR_LEVEL_SEMIALPHABETIC",
>                   [          i,         I,     paragraph,          none ]};
>     key <AD08>  { type[group1] = "FOUR_LEVEL_SEMIALPHABETIC",
>                   [  idotless,  Iabovedot,    apostrophe,    dead_caron ]};
> };
> ```
> 
> However, this only swaps the uppercase letters, so the `i` key is unchanged but
> the uppercase of that key is `I` like in English. However, this is usually not
> desired as this still reduces your typing speed (as the `iI` key is too far).

Let's create our own layout that does something similar but swaps the lowercase
letters instead. Here is the code for that:

```cpp
default partial
xkb_symbols "basic" {
    include "tr(basic)"

    name[Group1]="Turkish (i and ı swapped correctly)";

    key <AC11>  { type[group1] = "FOUR_LEVEL_SEMIALPHABETIC",
                [  idotless, Iabovedot, paragraph , none       ]};
    key <AD08>  { type[group1] = "FOUR_LEVEL_SEMIALPHABETIC",
                [  i       , I        , apostrophe, dead_caron ]};
};
```

The `default` key is needed because we are going to make this a standalone file.
Save this to `~/.config/xkb/symbols/tr-swapped-i` and you can tell your WM/DE
to use the `tr-swapped-i` XKB layout.

In Hyprland (The WM I use) you can do it like this:

```cpp
# In ~/.config/hypr/hyprland.conf
input {
  kb_layout = "tr-swapped-i"
}
```

That should swap the `ı` and `i` keys on your WM/DE successfully. However, we are not done
yet.

The TTY (swap to TTY 2 by doing `CTRL-ALT-F2`) still doesn't use this layout, which is a problem.
And it seems that the format the TTY uses is a little different.
Looking at the `kbd` package, it uses a format called `.map`.

Here is a `.map` file that overrides the `trq` layout and swaps the `ı` and `i` keys:

```cpp
include "/usr/share/keymaps/i386/qwerty/trq.map"

keycode 23 = i
	altgr       keycode 23 = +icircumflex
	altgr shift keycode 23 = +Icircumflex

keycode 40 = +dotlessi +Idotabove
```

Save it to the directory where you store your maps.

Note that the path `/usr/share/keymaps` might differ based on your distro. You
can check what it is it by doing `man loadkeys` and scrolling to the bottom.

After that, consult your distro's docs on how to change the system keyboard locale.

This is how it is done on NixOS:

```nix
{ pkgs, ... }: {
  console.keyMap = pkgs.writeText "trq-swapped-i.map" ''
    include "${pkgs.kbd}/share/keymaps/i386/qwerty/trq.map"

    keycode 23 = i
    	altgr       keycode 23 = +icircumflex
    	altgr shift keycode 23 = +Icircumflex

    keycode 40 = +dotlessi +Idotabove
  '';
}
```

And this is how it is done on Arch:

```shell
cat << :end
include "/usr/share/kbd/keymaps/i386/qwerty/trq.map"

keycode 23 = i
	altgr       keycode 23 = +icircumflex
	altgr shift keycode 23 = +Icircumflex

keycode 40 = +dotlessi +Idotabove
:end > /usr/share/kbd/keymaps/i386/qwerty/trq-swapped-i.map

localectl set-keymap --no-convert tr-swapped-i
```

After that, your TTY should also use this locale.

Gotta love having to mess with deep parts of your system because the Turkish-Q
layout did the big mistake of reusing the `i` key as `ı`!

At least I'll type and edit faster now...

[Here is the commit where I implemented this, might be worth a look if you use NixOS.](https://github.com/RGBCube/NCC/commit/6d18066eb5ccefa4539205f3d6721e4a8ff8b97e)
