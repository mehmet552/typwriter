Name:           typwriter
Version:        1.0.0
Release:        1%{?dist}
Summary:        Typewriter-themed focused writing application
License:        GPL-3.0-or-later
URL:            https://github.com/mehmet/typwriter
Source0:        %{url}/archive/v%{version}/%{name}-%{version}.tar.gz

BuildRequires:  rust >= 1.70
BuildRequires:  cargo
BuildRequires:  gtk4-devel >= 4.12
BuildRequires:  libadwaita-devel >= 1.4
BuildRequires:  alsa-lib-devel
BuildRequires:  pkg-config
BuildRequires:  gcc

Requires:       gtk4 >= 4.12
Requires:       libadwaita >= 1.4

%description
Typwriter is a typewriter-themed focused writing application.
It features realistic typewriter key sounds, ambient atmosphere
sounds (fireplace, rain, night, cafe), DOCX/TXT document support,
and visual effects that immerse you in a distraction-free writing environment.

Daktilo temalı odaklanma yazma uygulaması. Gerçekçi daktilo
sesleri, atmosfer ortamları, DOCX/TXT desteği ve görsel efektlerle
dikkat dağıtmayan bir yazma deneyimi sunar.

%prep
%autosetup -n %{name}-%{version}

%build
cargo build --release

%install
install -Dm755 target/release/%{name} %{buildroot}%{_bindir}/%{name}
install -Dm644 data/com.github.mehmet.typwriter.desktop %{buildroot}%{_datadir}/applications/com.github.mehmet.typwriter.desktop
install -Dm644 data/com.github.mehmet.typwriter.metainfo.xml %{buildroot}%{_metainfodir}/com.github.mehmet.typwriter.metainfo.xml
install -Dm644 data/com.github.mehmet.typwriter.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/com.github.mehmet.typwriter.svg

%files
%license LICENSE
%doc README.md
%{_bindir}/%{name}
%{_datadir}/applications/com.github.mehmet.typwriter.desktop
%{_metainfodir}/com.github.mehmet.typwriter.metainfo.xml
%{_datadir}/icons/hicolor/scalable/apps/com.github.mehmet.typwriter.svg

%changelog
* Sun Oct 04 2026 Mehmet - 1.0.0-1
- Initial release with typewriter audio, atmospheres, and DOCX/TXT support
