# 🖋️ Typwriter

Daktilo temalı odaklanma yazma uygulaması. / Typewriter-themed focused writing application.

[![License: GPL-3.0](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Platform: Linux](https://img.shields.io/badge/Platform-Linux-lightgrey.svg)]()

![Typwriter Screenshot](https://via.placeholder.com/800x450?text=Typwriter+Screenshot)

## 🌟 Özellikler / Features

- 🎹 **Gerçekçi Sesler / Realistic Sounds:** Her tuş vuruşunda tatmin edici daktilo sesleri. (Satisfying typewriter sounds with every keystroke.)
- 🌧️ **Atmosferler / Atmospheres:** Yazma havasına girmek için farklı ortamlar: (Different environments to get in the mood for writing:)
  - 🔥 Şömine / Fireplace: Sıcak bir parıltı ve çıtırtı. (Warm glow and crackle.)
  - 🌧️ Yağmur / Rain: Yağmurlu bir günün huzuru. (Peace of a rainy day.)
  - 🌙 Gece / Night: Karanlık ve sessiz. (Dark and quiet.)
  - ☕ Kafe / Cafe: Hafif bir kafe uğultusu. (Light cafe murmur.)
- 🖥️ **Odak Modu / Focus Mode:** Tam ekran özelliği ile dikkat dağıtıcı unsurları engelleyin. (Block distractions with full-screen feature.)

## 🚀 Kurulum / Installation

### Gereksinimler / Dependencies

- Rust (Cargo) 1.70+
- GTK4 development files (`gtk4-devel`, `libgtk-4-dev`)
- Libadwaita development files (`libadwaita-devel`, `libadwaita-1-dev`)
- ALSA development files (`alsa-lib-devel`, `libasound2-dev`)

### Evrensel Kurulum / Universal Installation (Linux)

Depoyu klonlayın ve kurulum betiğini çalıştırın: (Clone the repo and run the install script:)

```bash
git clone https://github.com/mehmet/typwriter.git
cd typwriter
chmod +x install.sh
./install.sh
```

### Manuel Derleme / Manual Build (Makefile)

```bash
make build
sudo make install
```

### Fedora RPM

```bash
rpmbuild -ba rpm/typwriter.spec
sudo dnf install ~/rpmbuild/RPMS/x86_64/typwriter-1.0.0-1.fc*.x86_64.rpm
```

## ⌨️ Kullanım / Usage

- Uygulamayı `typwriter` komutuyla veya menüden başlatın. (Launch with the `typwriter` command or from the menu.)
- Alt kısımdaki seçiciden atmosferi değiştirin. (Change the atmosphere from the selector at the bottom.)
- Tam ekran yapmak için **F11** tuşuna basın. (Press **F11** to toggle fullscreen.)

## 📝 Lisans / License

Bu proje GPL-3.0 veya daha yeni bir sürüm altında lisanslanmıştır. Detaylar için [LICENSE](LICENSE) dosyasına bakın.
(This project is licensed under GPL-3.0-or-later. See the [LICENSE](LICENSE) file for details.)

## 🤝 Katkıda Bulunma / Contributing

Hata bildirimleri ve pull request'ler her zaman memnuniyetle karşılanır! (Bug reports and pull requests are always welcome!)
