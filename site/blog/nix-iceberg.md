---
title: Explaining the Nix iceberg
description: And revealing how cursed Nix is.

date: 2024-04-15
draft: true

tags:
- nix
---

I was surfing the web a few weeks ago, and I came across
this iceberg chart:

![The Nix Iceberg](/assets/nix-iceberg.webp)

[Here's the original source for this image,
created by @leftpaddotpy, @puckipedia,
@wiggles and @qyriad on cohost.](https://cohost.org/leftpaddotpy/post/3885451-the-nix-iceberg)

In this post, I'll be explaining every item in this
iceberg with sufficient depth. Let's start:

# Tier 1: I use NixOS (BTW)

## IFD blocks evaulation

> IFD stands for import-from-derivation.

IFD is when you import a Nix expression
from a derivation in the Nix store.

For example:

```nix
let
  pkgs = import <nixpkgs> {};

  myNixExprDeriv = pkgs.runCommand "my-file" {} ''
    echo '{ a = "b"; }' > $out
  '';

  mySet = import myNixExprDeriv;
in mySet.a
```

This will evaluate to `"b"`.

So, what are we doing in this snippet?

1. Importing `<nixpkgs>` and getting the packages out of it.
2. Creating a derivation that runs an echo command, which
   writes a Nix expression to the output file.
3. Then we import the expression, forcing the derivation to
   be realized as we accessed the contents of it.

> Wait, what does _realization_ mean?

It means to actually build a `.drv` file, using the builder,
arguments and inputs described in it.

Nix does not realize derivations until you access the
contents of them or force them to be evaluated using the `:b`
command in the Nix REPL, see these two examples:

```nix
nix-repl> pkgs = import <nixpkgs> {}

nix-repl> pkgs.runCommand "foo" {} "echo 'bar' > $out"
«derivation /nix/store/h27fzbivcxw0cc1bxyyyqyivpw9rsz6k-foo.drv»
```

Here, it did create a `.drv` file. But that's it. There is no
`/nix/store/AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA-foo` with contents
`bar` to be seen.

```nix
nix-repl> :b pkgs.runCommand "foo" {} "echo 'bar' > $out"

This derivation produced the following outputs:
  out -> /nix/store/rxz2bswgx6wlkdxnrcbsb503r9a67wc2-foo
```

And here we force the derivation to be realized, which produces the output.

Where were we again? Right, the 3rd point:
`Then we import the expression, forcing the derivation to
be realized as we accessed the contents of it.`

The 3rd point is the important part. A typical Nix expression does
not depend on the output contents of any derivation, which in turn
makes evaluating a Nix expression not require realizing _any_ derivations.

But with IFD, you have to realize a derivation to even finish the
evaluation of your Nix expression. This will block Nix evaluation
for a long time, as Nix is evaluated on a single thread and
realizing the derivation needed takes a non-trivial amount of time.

TL;DR: IFD blocks evaluation because:

1. Evaluation is single threaded, so naturally everything blocks it.
2. You're trying to access a derivation _output_, so obviously
   you need to realize (build) it first.

## `nix-shell` and `nix shell` are completely different

`nix-shell` is the legacy version of `nix develop`, which
enters a devshell created by a Nix expression. It was (and
still is) very useful.

People then realized getting a devshell by passing in the packages
you wanted as command line arguments was really convenient,
which resulted in the creation of the `--packages/-p` argument for `nix-shell`

`nix-shell -p` is similar to `nix shell`. But they are not the same.

`nix-shell -p` creates a shell using the stdenv by calling `pkgs.mkShell`,
which includes all packages in the nixpkgs stdenv plus the ones you specified.

`nix shell` only appends the packages you passed in to the `PATH` environment
variable. It is much lighter, as a natural result of not using the stdenv.
It also isn't a questionable templated Nix expression and is implemented in
the Nix CLI natively.

## Hydra is 17,000 lines of Perl

As the title says, [Hydra](http://github.com/NixOS/hydra),
the Nix-based continuous build system is almost 17,000
lines of Perl.

Here is the `tokei` output for its GitHub repository:

| Language         | Files | Lines       | Code  | Comments | Blanks |
| ---------------- | ----- | ----------- | ----- | -------- | ------ |
| Autoconf         | 2     | 38          | 37    | 0        | 1      |
| Automake         | 13    | 175         | 150   | 0        | 25     |
| C++              | 9     | 4659        | 3448  | 406      | 805    |
| C++ Header       | 5     | 757         | 485   | 74       | 198    |
| CSS              | 3     | 505         | 388   | 35       | 82     |
| JavaScript       | 6     | 337         | 265   | 37       | 35     |
| Nix              | 38    | 2029        | 1732  | 77       | 220    |
| Nix (Markdown)   | 2     | 12          | 12    | 0        | 0      |
| Perl             | 125   | 16754 (!!!) | 12055 | 649      | 4050   |
| Python           | 1     | 35          | 25    | 1        | 9      |
| Shell            | 24    | 371         | 279   | 35       | 57     |
| Shell (Markdown) | 1     | 3           | 2     | 1        | 0      |
| SQL              | 85    | 1406        | 989   | 202      | 215    |
| SVG              | 6     | 6           | 6     | 0        | 0      |
| Plain Text       | 4     | 164         | 0     | 102      | 62     |
| YAML             | 1     | 1137        | 1094  | 0        | 43     |
| XML (Markdown)   | 2     | 25          | 25    | 0        | 0      |
| Markdown         | 18    | 2312        | 0     | 1744     | 568    |
| Markdown (Total) | 18    | 2352        | 39    | 1745     | 568    |
| Total            | 340   | 30685       | 20953 | 3362     | 6370   |

## Nix Pills

From <https://nixos.org/guides/nix-pills/>:

> This is a ported version of the Nix Pills, a series of blog posts written
> by Luca Bruno (aka Lethalman) and originally published in 2014 and 2015.
> It provides a tutorial introduction into the Nix package manager and Nixpkgs
> package collection, in the form of short chapters called 'pills'.
>
> Since the Nix Pills are considered a classic introduction to Nix, an effort
> to port them to the current format was led by Graham Christensen (aka grahamc
> / gchristensen) and other contributors in 2017.

## `inherit`

`inherit` is a keyword in the Nix language that brings a variable
into an attribute set. It can also be used in `let in`s.

Check out the
[Nix reference page](https://nixos.org/manual/nix/stable/language/constructs.html#inheriting-attributes)
that explains the keyword in depth.

## `nix-tree`

[`nix-tree`](https://github.com/utdemir/nix-tree) is a tool to interactively
browse dependency graphs of derivations. Made in Haskell, of course.

## `nix-diff`

[`nix-diff`](https://github.com/Gabriella439/nix-diff) is a tool to see how
two derivations differ with colored output. Again, in Haskell.

## `nix-shell -p` gives you a compiler

As mentioned in the `nix-shell and nix shell are completely different`
section, `nix-shell -p` is the nixpkgs stdenv plus your packages.

And since the stdenv includes a C compiler, so does the shell
you enter after calling `nix-shell -p hello`.

## `nix-output-monitor`

[`nix-output-monitor`](https://github.com/maralorn/nix-output-monitor),
also known as `NOM` is a neat visualizer for Nix builds.
See it in action: <https://asciinema.org/a/604200>

It is also programmed in Haskell. Whew.

## `nix-top`

[`nix-top`] is a simple Ruby script to help people
see what is building in the local Nix daemon. to help people
see what is building in the local Nix daemon.

## `--debugger`

The `--debugger` flag is used to halt evaulation and
enter the Nix REPL when evaluating a Nix file or expression.

You set breakpoints using the `builtins.break` function:

```nix
let
  foo = 123;
  bar = "baz";

  # Nix will stop right here, just before evaulating the attrset
  # passed into `builtins.break`. We should be able to access
  # `foo` and `bar`. But it doesn't work!
in builtins.break {
  inherit foo bar;
}
```

> Evaulate this file with `nix eval --debugger --file <filename>` and see.

It is also _supposed_ to bring the variables in the scope `break`
was called into the Nix REPL. However, this does not work. Keep on
reading and you'll see why & what do to do bypass this bug!

## `tvix`

[Tvix](https://tvix.dev/) is an alternate implementation of Nix written in Rust.

It aims to have a modular implementation while also reusing already-written
Nix crates in the Rust ecosystem so other people can reuse code instead of
reimplementing it! It is licensed under the GPLv3 license.

## Eelco's Thesis

Eelco's thesis is about The Purely Functional Software
Deployment Model. Which also happens to be about Nix.

You can read the thesis [here](https://edolstra.github.io/pubs/phd-thesis.pdf).

## Fixed-Output derivations do not rebuild with a changed URL

Fixed output derivations (also called FODs) do not get rebuilt
even if you change any inputs passed to them (a URL string is
also an input). The reason for this is simple.

Nix will see that the output is the same, and since there already
is a derivation with the same output in the Nix store, it will
assume it is cached and will use that derivation.

# Tier 2: Package Maintainer

## `github:boolean-option/true`

The [`boolean-option` GitHub organization](https://github.com/boolean-option)
allows flakes to be configured in "flake compile time". Let's say you have a
flake that provides a binary. Let's also assume you can run it with the
following Nix CLI invokation:

```shell
nix run github:me/hello-world
```

This is great, you are able to run the binary. But, there is no way for a flake to
accept any configuration arguments. If you wanted to run in debug mode, you have
to create another output (like `packages.x86_64-linux.{release,debug}`).
Same for compiling without support for X/Y/Z. This results in two to the N power
of outputs, where N is the feature toggle count.

A dumb flake input like `github:boolean-option/true` fixes this, even though
it is an ugly hack. You can do this in your flake:

```nix
{
  inputs = {
    nixpkgs.url    = "github:NixOS/nixpkgs/nixos-23.11";
    debug-mode.url = "github:boolean-option/false"; # Release by default!
  };

  outputs = { nixpkgs, debug-mode, ... }: let
    pkgs = import nixpkgs { system = "x86_64-linux"; };
  in {
    packages.x86_64-linux.hello = pkgs.callPackage ./hello { inherit debug-mode; };
  };
}
```

And override the `debug-mode` input like so, to run a debug binary instead:

```shell
nix run github:me/hello-world --override debug-mode github:boolean-option/true
```

[`nix-systems`](https://github.com/nix-systems/nix-systems) is the same idea
as `boolean-option`, but for systems instead.

[See some example usages here.](https://github.com/search?q=boolean-option+language%3ANix&type=code&l=Nix)

These hacks wouldn't be needed if Nix allowed users to put arbitrary values in
inputs - [in fact, there is an open issue from _2021_ that is still being actively
discussed](https://github.com/NixOS/nix/issues/5663) - but here we are.

## `''foo''\n'' == "foo\n"`

The Nix parser is very buggy, and this is one bug.

`''` is the character set used to escape `${` in
Nix indent strings (No, not multiline strings! All strings in Nix
are multiline.):

```nix
''
  export BAR_OR_BAZ=''${BAR:-$BAZ}
''
```

This results in the literal string `"export BAR_OR_BAZ=${BAR:-BAZ}"`, without
string interpolation.

Nix will ignore an invalid `\` escape after the `''` escape in an indent string.
Or if it is a valid one, it will just append the `\` escape to
the string, ignoring the `''` escape.

## `(x: x x) (x: x x)`

This expression is a way to make Nix recurse forever
and stack overflow. Nix can't detect it either, as the
evaluated thunk is always different.

## Derivations are just memoized `execve`

Derivations include all required information to build themselves.
This also includes output directories (except when they are content-addressed,
but that is for a future blog post!). You can dump a `.drv` file as JSON with the
`nix derivation show` command, like so:

<details>
<summary>Long command output</summary>

```json
❯ nix derivation show /nix/store/0aplz036lmggrryvx2xh87ci20hczijf-libsamplerate-0.1.9.drv^*

{
  "/nix/store/0aplz036lmggrryvx2xh87ci20hczijf-libsamplerate-0.1.9.drv": {
    "args": [
      "-e",
      "/nix/store/v6x3cs394jgqfbi0a42pam708flxaphh-default-builder.sh"
    ],
    "builder": "/nix/store/bm0gsz7di3d4q0gw1kk2pa06505b0wmn-bash-5.2p26/bin/bash",
    "env": {
      "__structuredAttrs": "",
      "bin": "/nix/store/r3n9n5483q2zprrrjj0f442n723dkzyk-libsamplerate-0.1.9-bin",
      "buildInputs": "/nix/store/4rbkn1f0px39n75zbib2f43i851vy0ay-libsndfile-1.2.2-dev",
      "builder": "/nix/store/bm0gsz7di3d4q0gw1kk2pa06505b0wmn-bash-5.2p26/bin/bash",
      "cmakeFlags": "",
      "configureFlags": "--disable-fftw",
      "depsBuildBuild": "",
      "depsBuildBuildPropagated": "",
      "depsBuildTarget": "",
      "depsBuildTargetPropagated": "",
      "depsHostHost": "",
      "depsHostHostPropagated": "",
      "depsTargetTarget": "",
      "depsTargetTargetPropagated": "",
      "dev": "/nix/store/ajfrbfsqbmxb4ypnmp39xxdpg9gplxbx-libsamplerate-0.1.9-dev",
      "doCheck": "",
      "doInstallCheck": "",
      "mesonFlags": "",
      "name": "libsamplerate-0.1.9",
      "nativeBuildInputs": "/nix/store/xpah4lnaggs6qg87pg1rd9his89acprm-pkg-config-wrapper-0.29.2",
      "out": "/nix/store/55mwzr1k14mryxnhzz6z3hzaimhl8bpn-libsamplerate-0.1.9",
      "outputs": "bin dev out",
      "patches": "",
      "pname": "libsamplerate",
      "postConfigure": "",
      "propagatedBuildInputs": "",
      "propagatedNativeBuildInputs": "",
      "src": "/nix/store/9jnvkn9wcac6r62mljq9fa9vvriyib1i-libsamplerate-0.1.9.tar.gz",
      "stdenv": "/nix/store/jiz7bpw8vqzq8ncm6nn4v94qyqm9qc2p-stdenv-linux",
      "strictDeps": "",
      "system": "i686-linux",
      "version": "0.1.9"
    },
    "inputDrvs": {
      "/nix/store/356i9xqk710rnmq6y6308sv880m88r7k-pkg-config-wrapper-0.29.2.drv": {
        "dynamicOutputs": {},
        "outputs": [
          "out"
        ]
      },
      "/nix/store/gfybzgm5p0hh7w7mdrz5xkr29dlsriih-libsamplerate-0.1.9.tar.gz.drv": {
        "dynamicOutputs": {},
        "outputs": [
          "out"
        ]
      },
      "/nix/store/jkfhhkxlbkfhmqhaccpmqdna01wzlb42-libsndfile-1.2.2.drv": {
        "dynamicOutputs": {},
        "outputs": [
          "dev"
        ]
      },
      "/nix/store/zlf7fmxbnq4k2xgngk0p953ywjqbci6f-stdenv-linux.drv": {
        "dynamicOutputs": {},
        "outputs": [
          "out"
        ]
      },
      "/nix/store/zx3fgspv17raqfb859qkpqnql2fschm0-bash-5.2p26.drv": {
        "dynamicOutputs": {},
        "outputs": [
          "out"
        ]
      }
    },
    "inputSrcs": [
      "/nix/store/v6x3cs394jgqfbi0a42pam708flxaphh-default-builder.sh"
    ],
    "name": "libsamplerate-0.1.9",
    "outputs": {
      "bin": {
        "path": "/nix/store/r3n9n5483q2zprrrjj0f442n723dkzyk-libsamplerate-0.1.9-bin"
      },
      "dev": {
        "path": "/nix/store/ajfrbfsqbmxb4ypnmp39xxdpg9gplxbx-libsamplerate-0.1.9-dev"
      },
      "out": {
        "path": "/nix/store/55mwzr1k14mryxnhzz6z3hzaimhl8bpn-libsamplerate-0.1.9"
      }
    },
    "system": "i686-linux"
  }
}
```
</details>

## `nixos-rebuild --fast --target-host`

The `--fast` flag in `nixos-rebuild` is an alias to `--no-build-nix`
which is explained in the man page like so:

> Normally, nixos-rebuild first builds the `nixUnstable` attribute in Nixpkgs,
> and uses the resulting instance of the Nix package manager to build the new
> system configuration. This is necessary if the NixOS modules use features not
> provided by the currently installed version of Nix. This option disables
> building a new Nix.

And the `--target-host` flag is also documented (rare!), like so:

> Specifies the NixOS target host. By setting this to something other than
> an empty string, the system activation will happen on the remote host
> instead of the local machine. The remote host needs to be accessible over
> ssh, and for the commands switch, boot and test you need root access.
>
> If `--build-host` is not explicitly specified or empty, building will take
> place locally.
>
> You can include a remote user name in the host name (user@host). You can
> also set ssh options by defining the `NIX_SSHOPTS` environment variable.
>
> Note that nixos-rebuild honors the nixpkgs.crossSystem setting of the
> given configuration but disregards the true architecture of the target
> host. Hence the nixpkgs.crossSystem setting has to match the target platform
> or else activation will fail.

## Nix supports floats

Yup, you heard it. Nix has floats, too!

Though, note that not every number in Nix is a float.
Integers in Nix are stored as 64-bit integers. Floats are also
64-bit. [Here's the Nix source code that denotes this](https://github.com/NixOS/nix/blob/d2a07a96ba6275e570b7d84092d08cbe85a2091b/src/libexpr/value.hh#L77-L78)

```nix
nix-repl> 0.1 + 0.2
0.3

nix-repl> 0.1 + 0.2 == 0.3
false

nix-repl> 0.2 + 0.2 == 0.4
true
```

## `attrset ? key` and `attrset ? "key"`

This syntax is a way to check for the existence of a key
in an attribute set.

`{ foo = 42; } ? foo` evaulates to `true`. The same applies for
`{ foo = 42; } ? "foo"`, which is just using a string identifier instead.

## Flakes invented for Target Corporation

[The development of flakes was partially funded by Target Corporation.](https://www.tweag.io/blog/2020-07-31-nixos-flakes/#conclusion)