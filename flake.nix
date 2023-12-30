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
      url                        = "github:RGBCube/FlakeTools";
      inputs.nixpkgs.follows     = "nixpkgs";
    };

    cargo2nix = {
      url                    = "github:cargo2nix/cargo2nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, tools, cargo2nix } @ inputs: tools.eachDefaultLinuxArch (system: let
    pkgs = import nixpkgs {
      inherit system;

      overlays = [ cargo2nix.overlays.default ];
    };

    rustPackages = pkgs.rustBuilder.makePackageSet {
      rustVersion = "1.76.0";
      rustChannel = "nightly";
      rustProfile = "minimal";
      packageFun  = import ./Cargo.nix;
    };
  in {
    devShells.${system}.default = rustPackages.workspaceShell {
      packages = [ cargo2nix.packages.${system}.default ];
    };

    packages.${system}.site    = rustPackages.workspace.site {};
    packages.${system}.default = self.packages.${system}.site;

    nixosModules.default = { config, lib, pkgs, ... }: with lib; let
      cfg = config.services.site;
    in {
      options = {
        services.site = {
          enable = mkEnableOption (mdDoc "site service");

          port = mkOption {
            type        = types.port;
            default     = 8080;
            example     = 80;
            description = mdDoc ''
              Specifies on which port the site service listens for connections.
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
            needsPrivilidges = cfg.port < 1024;
            capabilities     = [ "" ] ++ optionals needsPrivileges [ "CAP_NET_BIND_SERVICE" ];
            rootDirectory    = "/run/site";
          in {
            ExecStart               = "${self.packages.${pkgs.system}.site}/bin/site --port ${cfg.port} --log-level ${cfg.logLevel}";
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
          optionals cfg.openFirewall [ cfg.port ];
      };
    };
  });
}
