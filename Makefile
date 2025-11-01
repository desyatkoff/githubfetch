BINDIR = /usr/bin
TARGET = githubfetch

all: build install

build:
	cargo build --release --verbose

clean:
	cargo clean --verbose
	rm -fv $(BINDIR)/$(TARGET)

install:
	install -D -m755 -v ./target/release/$(TARGET) $(BINDIR)/$(TARGET)

uninstall:
	rm -fv $(BINDIR)/$(TARGET)

.PHONY: all build clean install uninstall
