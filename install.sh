#!/bin/bash
set -e

echo "🖋️  Typwriter Kurulum / Installation"
echo "====================================="
echo ""

# Check dependencies
check_dep() {
    if ! command -v "$1" &> /dev/null; then
        echo "❌ $1 bulunamadı / not found"
        return 1
    fi
    return 0
}

# Check for Rust/Cargo
if ! check_dep cargo; then
    echo "Rust kurulumu gerekli / Rust installation required:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Check for pkg-config
check_dep pkg-config || {
    echo "pkg-config kurulumu gerekli / pkg-config required"
    exit 1
}

# Check GTK4 development files
if ! pkg-config --exists gtk4; then
    echo "❌ GTK4 development kütüphaneleri bulunamadı"
    echo ""
    echo "Kurulum / Install:"
    echo "  Fedora/RHEL:  sudo dnf install gtk4-devel libadwaita-devel alsa-lib-devel"
    echo "  Ubuntu/Debian: sudo apt install libgtk-4-dev libadwaita-1-dev libasound2-dev"
    echo "  Arch:          sudo pacman -S gtk4 libadwaita alsa-lib"
    echo "  openSUSE:      sudo zypper install gtk4-devel libadwaita-devel alsa-devel"
    exit 1
fi

echo "✅ Bağımlılıklar tamam / Dependencies OK"
echo ""
echo "🔨 Derleniyor / Building..."
cargo build --release

echo ""
echo "📦 Kuruluyor / Installing..."
PREFIX="${PREFIX:-/usr/local}"
sudo install -Dm755 target/release/typwriter "${PREFIX}/bin/typwriter"
sudo install -Dm644 data/com.github.mehmet.typwriter.desktop "${PREFIX}/share/applications/com.github.mehmet.typwriter.desktop"
sudo install -Dm644 data/com.github.mehmet.typwriter.metainfo.xml "${PREFIX}/share/metainfo/com.github.mehmet.typwriter.metainfo.xml"
sudo install -Dm644 data/com.github.mehmet.typwriter.svg "${PREFIX}/share/icons/hicolor/scalable/apps/com.github.mehmet.typwriter.svg"

# Güncelleme ikon önbelleği
if command -v gtk-update-icon-cache &> /dev/null; then
    sudo gtk-update-icon-cache -f -t "${PREFIX}/share/icons/hicolor" 2>/dev/null || true
fi

echo ""
echo "✅ Kurulum tamamlandı! / Installation complete!"
echo "🖋️  'typwriter' komutu ile veya uygulama menünüzden çalıştırabilirsiniz!"
