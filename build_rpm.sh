#!/bin/bash
set -e

echo "📦 RPM paketleme hazırlanıyor..."

TOPDIR="$(pwd)/rpmbuild"
mkdir -p "$TOPDIR"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

# Kodları bir araya getirip tarball yap
tar -czf "$TOPDIR/SOURCES/typwriter-1.0.0.tar.gz" \
    --exclude-vcs \
    --exclude="target" \
    --exclude="rpmbuild" \
    --exclude="dist" \
    --transform 's,^,typwriter-1.0.0/,' \
    .

# Spec dosyasını kopyala
cp rpm/typwriter.spec "$TOPDIR/SPECS/"

# RPM'i derle
rpmbuild --define "_topdir $TOPDIR" --define "_tmppath /tmp" -ba "$TOPDIR/SPECS/typwriter.spec"

mkdir -p dist
cp -v "$TOPDIR"/RPMS/x86_64/typwriter-1.0.0-1*.rpm dist/
rm -rf "$TOPDIR"

echo "✅ RPM başarıyla oluşturuldu ve dist/ dizinine kopyalandı!"
echo "📍 Konum: dist/typwriter-1.0.0-1.fc44.x86_64.rpm"
