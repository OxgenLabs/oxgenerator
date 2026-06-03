#!/usr/bin/env sh
set -eu

REPO="OxgeneratorLabs/oxgenerator"
BIN_NAME="oxgen"
INSTALL_DIR="${OXGEN_INSTALL_DIR:-$HOME/.local/bin}"

detect_target() {
    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
        Linux)
            os_target="unknown-linux-gnu"
            archive_ext="tar.gz"
            ;;
        Darwin)
            os_target="apple-darwin"
            archive_ext="tar.gz"
            ;;
        *)
            echo "unsupported operating system: $os" >&2
            exit 1
            ;;
    esac

    case "$arch" in
        x86_64 | amd64)
            arch_target="x86_64"
            ;;
        arm64 | aarch64)
            arch_target="aarch64"
            ;;
        *)
            echo "unsupported architecture: $arch" >&2
            exit 1
            ;;
    esac

    TARGET="${arch_target}-${os_target}"
    ARCHIVE_EXT="$archive_ext"
}

download_latest_release() {
    tmp_dir="$(mktemp -d)"
    archive_name="${BIN_NAME}-${TARGET}.${ARCHIVE_EXT}"
    download_url="https://github.com/${REPO}/releases/latest/download/${archive_name}"

    echo "Downloading ${BIN_NAME} from ${download_url}"

    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$download_url" -o "$tmp_dir/$archive_name"
    elif command -v wget >/dev/null 2>&1; then
        wget -q "$download_url" -O "$tmp_dir/$archive_name"
    else
        echo "curl or wget is required to install ${BIN_NAME}" >&2
        exit 1
    fi

    tar -xzf "$tmp_dir/$archive_name" -C "$tmp_dir"
}

install_binary() {
    mkdir -p "$INSTALL_DIR"

    if [ ! -f "$tmp_dir/$BIN_NAME" ]; then
        echo "archive does not contain ${BIN_NAME}" >&2
        exit 1
    fi

    chmod +x "$tmp_dir/$BIN_NAME"
    mv "$tmp_dir/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"

    echo "${BIN_NAME} installed to ${INSTALL_DIR}/${BIN_NAME}"

    case ":$PATH:" in
        *":$INSTALL_DIR:"*) ;;
        *)
            echo ""
            echo "warning: ${INSTALL_DIR} is not in your PATH"
            echo "add this line to your shell config:"
            echo "export PATH=\"${INSTALL_DIR}:\$PATH\""
            ;;
    esac
}

detect_target
download_latest_release
install_binary

echo ""
"$INSTALL_DIR/$BIN_NAME" --version
