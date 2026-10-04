#!/bin/bash
set -e

echo "📦 Typwriter - Tüm Dağıtımlar İçin Paketleme Başlatılıyor..."
echo "=========================================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

export PATH="$HOME/.cargo/bin:$PATH"

# 1. Optimize sürüm derlemesi
echo "🔨 Optimize ikili dosya derleniyor..."
cargo build --release

mkdir -p dist

# 2. Fedora / RHEL / openSUSE için RPM Paketi
echo "📦 1/3 - RPM paketi oluşturuluyor (Fedora, RHEL, openSUSE)..."
./build_rpm.sh

# 3. Ubuntu / Debian / Linux Mint için DEB Paketi
echo "📦 2/3 - DEB paketi oluşturuluyor (Ubuntu, Debian, Mint)..."
DEB_DIR="/tmp/typwriter-deb-build"
rm -rf "$DEB_DIR"
mkdir -p "$DEB_DIR/DEBIAN"
mkdir -p "$DEB_DIR/usr/bin"
mkdir -p "$DEB_DIR/usr/share/applications"
mkdir -p "$DEB_DIR/usr/share/metainfo"
mkdir -p "$DEB_DIR/usr/share/icons/hicolor/scalable/apps"

install -Dm755 target/release/typwriter "$DEB_DIR/usr/bin/typwriter"
install -Dm644 data/com.github.mehmet.typwriter.desktop "$DEB_DIR/usr/share/applications/"
install -Dm644 data/com.github.mehmet.typwriter.metainfo.xml "$DEB_DIR/usr/share/metainfo/"
install -Dm644 data/com.github.mehmet.typwriter.svg "$DEB_DIR/usr/share/icons/hicolor/scalable/apps/"

cat << 'CTRL' > "$DEB_DIR/DEBIAN/control"
Package: typwriter
Version: 1.0.0
Section: utils
Priority: optional
Architecture: amd64
Maintainer: Mehmet <mehmet@localhost>
Depends: libgtk-4-1 (>= 4.12) | libgtk-4-bin, libadwaita-1-0 (>= 1.4), libasound2 (>= 1.1)
Homepage: https://github.com/mehmet/typwriter
Description: Typewriter-themed focused writing application
 Daktilo temali odaklanma yazma uygulamasi. Gercekci daktilo
 sesleri, atmosfer ortamlari, DOCX/TXT destegi ve gorsel efektlerle
 dikkat dagitmayan bir yazma deneyimi sunar.
CTRL

DEB_TMP="/tmp/deb-pack-$$"
mkdir -p "$DEB_TMP"
echo "2.0" > "$DEB_TMP/debian-binary"
tar --numeric-owner --owner=0 --group=0 -czf "$DEB_TMP/control.tar.gz" -C "$DEB_DIR/DEBIAN" .
tar --numeric-owner --owner=0 --group=0 -czf "$DEB_TMP/data.tar.gz" --exclude='./DEBIAN' -C "$DEB_DIR" .
(cd "$DEB_TMP" && ar rc "$SCRIPT_DIR/dist/typwriter_1.0.0-1_amd64.deb" debian-binary control.tar.gz data.tar.gz)
rm -rf "$DEB_DIR" "$DEB_TMP"

# 4. Arch Linux, Manjaro ve Diğer Tüm Dağıtımlar İçin Evrensel Taşınabilir Paket (.tar.gz)
echo "📦 3/3 - Evrensel taşınabilir paket oluşturuluyor (Arch, Manjaro, Evrensel)..."
PORT_DIR="/tmp/typwriter-portable-1.0.0"
rm -rf "$PORT_DIR"
mkdir -p "$PORT_DIR"

install -Dm755 target/release/typwriter "$PORT_DIR/typwriter"
install -Dm644 data/com.github.mehmet.typwriter.desktop "$PORT_DIR/"
install -Dm644 data/com.github.mehmet.typwriter.svg "$PORT_DIR/"
install -Dm644 README.md "$PORT_DIR/"
install -Dm644 LICENSE "$PORT_DIR/"

cat << 'INST' > "$PORT_DIR/install.sh"
#!/bin/bash
set -e

echo "🖋️ Typwriter Taşınabilir Kurulum / Portable Installation"
echo "========================================================"

PREFIX="${PREFIX:-$HOME/.local}"
echo "Kurulum konumu: $PREFIX"

mkdir -p "$PREFIX/bin"
mkdir -p "$PREFIX/share/applications"
mkdir -p "$PREFIX/share/icons/hicolor/scalable/apps"

install -m755 typwriter "$PREFIX/bin/typwriter"
install -m644 com.github.mehmet.typwriter.desktop "$PREFIX/share/applications/"
install -m644 com.github.mehmet.typwriter.svg "$PREFIX/share/icons/hicolor/scalable/apps/"

if command -v update-desktop-database &> /dev/null; then
    update-desktop-database "$PREFIX/share/applications" 2>/dev/null || true
fi

echo "✅ Kurulum başarıyla tamamlandı!"
echo "Uygulama menüsünden veya '$PREFIX/bin/typwriter' komutuyla başlatabilirsiniz."
INST
chmod +x "$PORT_DIR/install.sh"

cat << 'UNINST' > "$PORT_DIR/uninstall.sh"
#!/bin/bash
set -e

PREFIX="${PREFIX:-$HOME/.local}"
rm -f "$PREFIX/bin/typwriter"
rm -f "$PREFIX/share/applications/com.github.mehmet.typwriter.desktop"
rm -f "$PREFIX/share/icons/hicolor/scalable/apps/com.github.mehmet.typwriter.svg"

echo "✅ Typwriter başarıyla kaldırıldı."
UNINST
chmod +x "$PORT_DIR/uninstall.sh"

tar -czf "$SCRIPT_DIR/dist/typwriter-1.0.0-linux-x86_64.tar.gz" -C /tmp typwriter-portable-1.0.0
rm -rf "$PORT_DIR"

echo ""
echo "🎉 Tüm paketler hazır! dist/ klasörü içeriği:"
ls -lh "$SCRIPT_DIR/dist/"
