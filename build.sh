#!/bin/bash

set -e

BASEDIR=$(dirname $(readlink -f $0))
OUTPUTDIR=$BASEDIR/output
TMPDIR=$BASEDIR/tmp
USRDIR=$TMPDIR/usr
BINDIR=$USRDIR/bin
LICDIR=$USRDIR/share/licenses/dua

umask 022
rm -rf $OUTPUTDIR $USRDIR
mkdir -p $OUTPUTDIR
mkdir -p $BINDIR
mkdir -p $TMPDIR
mkdir -p $LICDIR

cargo build --release --target x86_64-unknown-linux-gnu
cp target/release/dua $BINDIR
cp LICENSE $LICDIR

version=$(git describe --tags 2>/dev/null) || version=nover

typeset -a CARGS=(
    -a x86_64
    -s dir
    -n dua
    -v"$version"
    --description "Disk Usage Analyzer"
    -m "szekeres@iii.hu"
    --deb-no-default-config-files
    --vendor "Istvan Szekeres"
    --url "https://github.com/Pistahh/dua"
    -C $TMPDIR
    -p $OUTPUTDIR
)

typeset -a DIRS=( usr )

typeset -a DEB_DEPS=()
typeset -a PACMAN_DEPS=()


(cd $BASEDIR && fpm -t deb    "${CARGS[@]}" "${DEB_DEPS[@]}"    "${DIRS[@]}")
(cd $BASEDIR && fpm -t pacman "${CARGS[@]}" "${PACMAN_DEPS[@]}" "${DIRS[@]}")

