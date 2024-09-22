#!/bin/bash

# Upewnij się, że skrypt jest uruchamiany jako root
if [ "$EUID" -ne 0 ]; then
  echo "Proszę uruchomić jako root (sudo)."
  exit
fi

# Kompilacja programu
echo "Kompilowanie programu..."
cargo build --release

# Sprawdzenie, czy kompilacja się powiodła
if [ $? -ne 0 ]; then
  echo "Kompilacja nie powiodła się."
  exit 1
fi

# Skopiuj plik binarny do /usr/local/bin
echo "Instalowanie programu..."
cp target/release/rustie /usr/local/bin/

# Sprawdzenie, czy kopiowanie się powiodło
if [ $? -eq 0 ]; then
  echo "Instalacja zakończona pomyślnie! Możesz teraz uruchomić program jako 'rustie'."
else
  echo "Instalacja nie powiodła się."
  exit 1
fi
