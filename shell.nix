{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  name = "tauri-build-env";

  buildInputs = [
    pkgs.pkg-config # Required for finding system libraries
    pkgs.gtk3 # GTK for Tauri
    pkgs.gdk-pixbuf # GDK for image handling
    pkgs.libsoup # Required for soup2-sys
    pkgs.xorg.libX11 # X11 library for x11 crate
    pkgs.xorg.libxcb # XCB library (dependency of libX11)
    pkgs.xorg.libXrandr # Xrandr extension (commonly required)
    pkgs.xorg.libXtst # Xtst extension (optional but useful)
    pkgs.xorg.libXcursor # Xcursor (optional, for cursor management)
    pkgs.webkitgtk # Provides JavaScriptCoreGTK and WebKitGTK
  ];

  # Optionally set PKG_CONFIG_PATH explicitly if needed
  shellHook = ''
    export PKG_CONFIG_PATH=$(pkg-config --variable pc_path pkg-config)
  '';
}
