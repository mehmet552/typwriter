#  Typwriter

Daktilo temalı odaklanma yazma uygulaması. / Typewriter-themed focused writing application.

[![License: GPL-3.0](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Platform: Linux](https://img.shields.io/badge/Platform-Linux-lightgrey.svg)]()

![Typwriter Screenshot](https://via.placeholder.com/800x450?text=Typwriter+Screenshot)

##  Özellikler / Features

- **Gerçekçi Daktilo Mekaniği / Authentic Mechanical Typewriter:**
  - Gerçekçi daktilo tuş, geri silme (backspace) ve satır başı kolu (carriage return) sesleri.
  - **Mekanik Satır Sonu Kilidi (Margin Lock):** Satır sonuna geldiğinizde zil çalar ve Enter'a basıp alt satıra geçene kadar yazma kilitlenir (klasik mekanik daktilolardaki gibi).
  - **Kağıt Formatı Seçimi:** Standart **A4** veya **Roman (Novel)** formatı seçimi ve gerçek sayfa kenar boşlukları.
- **Atmosferler / Atmospheres:** Yazma havasına girmek için kesintisiz döngüde çalışan ortam sesleri:
  - Şömine / Fireplace: Sıcak bir parıltı ve şömine çıtırtısı.
  - Yağmur / Rain: Huzurlu yağmur sesi.
  - Gece / Night: Dingin gece ambiyansı ve cırcır böcekleri.
  - Caz / Jazz: Plak cızırtılı nostaljik caz melodisi.
- **Belge Uyumluluğu / Document Support:**
  - Microsoft Word `.docx` ve düz metin `.txt` formatında belgeleri açma, düzenleme ve kaydetme.
- **Odaklanma Modu / Focus Mode:**
  - Tam ekran modu (**F11**) ile dikkat dağıtıcı tüm unsurları ortadan kaldırın.
- **Gelişmiş Ayarlar / Settings:**
  - Kağıt formatı seçimi, daktilo ve ambiyans ses seviyesi ayarları.

##  Kurulum & Paketler / Packages & Installation

En son hazır paketleri [GitHub Releases](https://github.com/mehmet552/typwriter/releases) sayfasından indirebilir veya yerel `dist/` klasöründen kurabilirsiniz:

### 1. Fedora / RHEL / openSUSE (.rpm)
```bash
sudo dnf install ./dist/typwriter-1.0.0-1.fc44.x86_64.rpm
```

### 2. Ubuntu / Debian / Linux Mint / Pop!_OS (.deb)
```bash
sudo apt install ./dist/typwriter_1.0.0-1_amd64.deb
```

### 3. Arch Linux / Manjaro / Evrensel Taşınabilir (.tar.gz)
```bash
tar -xzf ./dist/typwriter-1.0.0-linux-x86_64.tar.gz
cd typwriter-portable-1.0.0
./install.sh
```

---

### Kaynak Koddan Derleme / Build From Source

Gereksinimler (Dependencies):
- Rust (Cargo) 1.70+
- GTK4 (`gtk4-devel` / `libgtk-4-dev`)
- Libadwaita (`libadwaita-devel` / `libadwaita-1-dev`)
- ALSA (`alsa-lib-devel` / `libasound2-dev`)

Tüm dağıtımlar için paketleri tek seferde üretmek için:
```bash
./package.sh
```

Manuel derleme:
```bash
cargo build --release
```

## ⌨️ Kullanım / Usage

- Uygulamayı `typwriter` komutuyla veya menüden başlatın. (Launch with the `typwriter` command or from the menu.)
- Alt kısımdaki seçiciden atmosferi değiştirin. (Change the atmosphere from the selector at the bottom.)
- Tam ekran yapmak için **F11** tuşuna basın. (Press **F11** to toggle fullscreen.)

##  Lisans / License

Bu proje GPL-3.0 veya daha yeni bir sürüm altında lisanslanmıştır. Detaylar için [LICENSE](LICENSE) dosyasına bakın.
(This project is licensed under GPL-3.0-or-later. See the [LICENSE](LICENSE) file for details.)

## Katkıda Bulunma / Contributing

Hata bildirimleri ve pull request'ler her zaman memnuniyetle karşılanır! (Bug reports and pull requests are always welcome!)
