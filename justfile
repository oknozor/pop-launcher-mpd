all:
    cargo build --release
    mkdir -p ~/.local/share/pop-launcher/plugins/mpd
    install -Dm0755 target/release/pop-launcher-mpd-plugin ~/.local/share/pop-launcher/plugins/mpd/mpd
    install -Dm644 plugin.ron ~/.local/share/pop-launcher/plugins/mpd/plugin.ron
