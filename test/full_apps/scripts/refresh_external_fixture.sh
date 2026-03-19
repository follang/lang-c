#!/bin/sh
set -eu

usage() {
    cat <<'EOF'
Usage:
  refresh_external_fixture.sh list
  refresh_external_fixture.sh show <fixture>

Known fixtures:
  musl-stdint
EOF
}

show_musl_stdint() {
    cat <<'EOF'
fixture=musl-stdint
project=musl
version=v1.2.5
license=MIT
upstream=https://git.musl-libc.org/cgit/musl/
files=include/stdint.h;arch/x86_64/bits/stdint.h;arch/x86_64/bits/alltypes.h.in;include/alltypes.h.in;COPYRIGHT
target=test/full_apps/external/musl/stdint
EOF
}

if [ "$#" -lt 1 ]; then
    usage
    exit 1
fi

case "$1" in
    list)
        echo musl-stdint
        ;;
    show)
        if [ "$#" -ne 2 ]; then
            usage
            exit 1
        fi
        case "$2" in
            musl-stdint)
                show_musl_stdint
                ;;
            *)
                echo "unknown fixture: $2" >&2
                exit 1
                ;;
        esac
        ;;
    *)
        usage
        exit 1
        ;;
esac
