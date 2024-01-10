# RGBCube's Homepage

The official website and link portal of RGBCube and his work.

## Running

The flake provides a NixOS module, you can use it by adding the flake to your
inputs and then adding `site.nixosModules.default` to the `modules` parameter
on the call set to `nixpkgs.lib.nixosSystem`.

Check out the options to configure the service.

## Options

### `services.site.enable`

Enables the site service.

Default: false.

### `services.site.port`

Specifies on which port the site service listens for connections.

Default: 8080.

### `services.site.logLevel`

Specifies the log level that the site service will log stuff with.

Default: info.

### `services.site.openFirewall`

Whether to open the firewall port for the tcp listener.

Default: false.

## License

All the markdown files under the `src/` directory are licensed
under [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/).

The other code is licensed under the GPU GPL V3 (`LICENSE_GPL.md`):

```
Copyright (C) 2023-present  RGBCube

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```
