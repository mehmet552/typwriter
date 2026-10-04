#!/bin/bash
set -e

echo "📦 RPM paketleme hazırlanıyor..."

# rpmbuild dizinlerini oluştur
mkdir -p ~/rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

# Kodları bir araya getirip tarball yap
cd /home/mehmet/projoler/typwriter
tar -czvf ~/rpmbuild/SOURCES/typwriter-1.0.0.tar.gz \
    --exclude-vcs \
    --exclude="target" \
    --transform 's,^,typwriter-1.0.0/,' \
    .

# Spec dosyasını kopyala
cp rpm/typwriter.spec ~/rpmbuild/SPECS/

# RPM'i derle
rpmbuild -ba ~/rpmbuild/SPECS/typwriter.spec

echo "✅ RPM başarıyla oluşturuldu!"
echo "📍 Konum: ~/rpmbuild/RPMS/x86_64/"
