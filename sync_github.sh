#!/bin/bash
set -e

echo "🚀 Typwriter GitHub Senkronizasyonu..."

# 1. Güncellenen dosyaları git takibine al
git add .gitignore Cargo.toml README.md data/com.github.mehmet.typwriter.metainfo.xml package.sh rpm/typwriter.spec

# 2. Temiz commit oluştur
git commit -m "chore: update metadata, .gitignore and documentation for GitHub" || echo "Değişiklik yok veya zaten commit edildi."

# 3. Ana dalı 'main' olarak adlandır
git branch -M main

# 4. Uzak depoyu bağla
git remote remove origin 2>/dev/null || true
git remote add origin https://github.com/mehmet552/typwriter.git

echo ""
echo "✅ Yerel git deposu hazırlandı."
echo "📤 GitHub'a temiz push yapmak için:"
echo "   git push -u origin main --force"
