PREFIX ?= /usr/local
BINDIR = $(PREFIX)/bin
DATADIR = $(PREFIX)/share

.PHONY: all build install uninstall clean

all: build

build:
	cargo build --release

install: build
	install -Dm755 target/release/typwriter $(DESTDIR)$(BINDIR)/typwriter
	install -Dm644 data/com.github.mehmet.typwriter.desktop $(DESTDIR)$(DATADIR)/applications/com.github.mehmet.typwriter.desktop
	install -Dm644 data/com.github.mehmet.typwriter.metainfo.xml $(DESTDIR)$(DATADIR)/metainfo/com.github.mehmet.typwriter.metainfo.xml
	install -Dm644 data/com.github.mehmet.typwriter.svg $(DESTDIR)$(DATADIR)/icons/hicolor/scalable/apps/com.github.mehmet.typwriter.svg

uninstall:
	rm -f $(DESTDIR)$(BINDIR)/typwriter
	rm -f $(DESTDIR)$(DATADIR)/applications/com.github.mehmet.typwriter.desktop
	rm -f $(DESTDIR)$(DATADIR)/metainfo/com.github.mehmet.typwriter.metainfo.xml
	rm -f $(DESTDIR)$(DATADIR)/icons/hicolor/scalable/apps/com.github.mehmet.typwriter.svg

clean:
	cargo clean
