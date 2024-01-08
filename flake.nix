{
  description = "The official website and link portal of RGBCube and his work.";

  nixConfig = {
    extra-substituters        = "https://cache.garnix.io/";
    extra-trusted-public-keys = "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g=";
  };

  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-unstable";
    };

    tools = {
      url                    = "github:RGBCube/FlakeTools";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane = {
      url                    = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url                              = "github:nix-community/fenix";
      inputs.nixpkgs.follows           = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, tools, fenix, advisory-db, ... } @ inputs: tools.eachDefaultLinuxArch (system: let
    lib  = nixpkgs.lib;

    toolchain = fenix.packages.${system}.complete.withComponents [
      "cargo"
      "clippy"
      "rust-src"
      "rustc"
      "rustfmt"
    ];

    crane = inputs.crane.lib.${system}.overrideToolchain toolchain;

    cssFilter   = path: type: builtins.match ".*css$" path != null;
    gifFilter   = path: type: builtins.match ".*gif$" path != null;
    jsFilter    = path: type: builtins.match ".*js$" path != null;
    mdFilter    = path: type: builtins.match ".*md$" path != null;
    woff2Filter = path: type: builtins.match ".*woff2$" path != null;

     src = lib.cleanSourceWith {
       src    = crane.path ./.;
       filter = path: type: (crane.filterCargoSources path type)
        || (cssFilter   path type)
        || (gifFilter   path type)
        || (jsFilter    path type)
        || (mdFilter    path type)
        || (woff2Filter path type);
    };

    srcArgs = {
      inherit src;
    };

    commonArgs = srcArgs // {
      strictDeps = true;
    };

    cargoArtifacts = crane.buildDepsOnly commonArgs;

    site = crane.buildPackage (commonArgs // {
      inherit cargoArtifacts;
    });
  in {
    devShells.${system}.default = crane.devShell {};

    checks.${system} = {
      inherit site;

      clippy = crane.cargoClippy (commonArgs // {
        inherit cargoArtifacts;

        cargoClippyExtraArgs = "--all-targets -- --deny warnings";
      });

      fmt = crane.cargoFmt srcArgs;

      audit = crane.cargoAudit (srcArgs // {
        inherit advisory-db;
      });
    };

    packages.${system} = {
      inherit site;

      default = site;
    };

    nixosModules.default = { config, lib, pkgs, ... }: with lib; let
      cfg = config.services.site;
    in {
      options = {
        services.site = {
          enable = mkEnableOption (mdDoc "site service");

          certificate = mkOption {
            type        = types.nullOr types.path;
            default     = null;
            example     = "/path/to/cert.pem";
            description = mdDoc ''
              The path to the SSL certificate the site will use.
            '';
          };

          key = mkOption {
            type        = types.nullOr types.path;
            default     = null;
            example     = "/path/to/key.pem";
            description = mdDoc ''
              The path to the SSL key the site will use.
            '';
          };

          httpPort = mkOption {
            type        = types.port;
            default     = 8080;
            example     = 80;
            description = mdDoc ''
              Specifies on which port the site service listens for HTTP connections.
            '';
          };

          httpsPort = mkOption {
            type        = types.port;
            default     = 8443;
            example     = 80;
            description = mdDoc ''
              Specifies on which port the site service listens for HTTPS connections.
            '';
          };

          logLevel = mkOption {
            type        = types.enum [ "off" "error" "warn" "info" "debug" "trace" ];
            default     = "info";
            example     = "warn";
            description = mdDoc ''
              Specifies the log level that the site service will log stuff with.
            '';
          };

          openFirewall = mkOption {
            type        = types.bool;
            default     = false;
            description = mdDoc ''
              Whether to open the firewall port for the tcp listener.
            '';
          };
        };
      };

      config = mkIf cfg.enable {
        systemd.services.site = {
          description = "RGBCube's Homepage";
          requires    = [ "network.target" ];
          wantedBy    = [ "multi-user.target" ];

          serviceConfig = let
            arguments = [
              "--http-port" cfg.httpPort
              "--https-port" cfg.httpsPort
              "--log-level" cfg.logLevel
            ] ++ (optionals (cfg.certificate != null) [
              "--certificate" cfg.certificate
            ]) ++ (optionals (cfg.key != null) [
              "--key" cfg.key
            ]);
            needsPrivilidges = cfg.httpPort < 1024 || cfg.httpsPort < 1024;
            capabilities     = [ "" ] ++ optionals needsPrivilidges [ "CAP_NET_BIND_SERVICE" ];
            rootDirectory    = "/run/site";
          in {
            ExecStart               = "${self.packages.${pkgs.system}.site}/bin/site " + (concatStringsSep " " arguments);
            Restart                 = "always";
            DynamicUser             = true;
            RootDirectory           = rootDirectory;
            BindReadOnlyPaths       = [ builtins.storeDir ];
            InaccessiblePaths       = [ "-+${rootDirectory}"];
            RuntimeDirectory        = builtins.baseNameOf rootDirectory;
            RuntimeDirectoryMode    = 700;
            AmbientCapabilities     = capabilities;
            CapabilityBoundingSet   = capabilities;
            UMask                   = "0077";
            LockPersonality         = true;
            MemoryDenyWriteExecute  = true;
            NoNewPrivileges         = true;
            PrivateDevices          = true;
            PrivateTmp              = true;
            PrivateUsers            = !needsPrivileges;
            ProtectClock            = true;
            ProtectControlGroups    = true;
            ProtectHome             = true;
            ProtectHostname         = true;
            ProtectKernelLogs       = true;
            ProtectKernelModules    = true;
            ProtectKernelTunables   = true;
            ProtectSystem           = "strict";
            ProtectProc             = "noaccess";
            ProcSubset              = "pid";
            RemoveIPC               = true;
            RestrictAddressFamilies = [ "AF_INET" "AF_INET6" ];
            RestrictNamespaces      = true;
            RestrictRealtime        = true;
            RestrictSUIDSGID        = true;
            SystemCallArchitectures = "native";
            SystemCallFilter        = [ "@system-service" "~@resources" "~@privileged" ];
          };
        };

        networking.firewall.allowedTCPPorts =
          optionals cfg.openFirewall [ cfg.httpPort cfg.httpsPort ];
      };
    };
  });
}
