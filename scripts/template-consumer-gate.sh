#!/bin/sh
set -eu

atlas_root="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
template_root="$atlas_root/../template-atlas"
capture=0

usage() {
  cat <<'EOF'
Usage: sh scripts/template-consumer-gate.sh [--template-root PATH] [--capture]

Compile the four template-atlas applications against this Atlas checkout.
With --capture, also render every external consumer state into
target/template-consumer-captures/ for visual review.
EOF
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --template-root)
      [ "$#" -ge 2 ] || { usage >&2; exit 2; }
      template_root="$2"
      shift 2
      ;;
    --capture)
      capture=1
      shift
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      usage >&2
      exit 2
      ;;
  esac
done

[ -d "$template_root/slint" ] || {
  echo "template-atlas checkout not found: $template_root" >&2
  exit 1
}

git -C "$template_root" rev-parse --verify HEAD >/dev/null 2>&1 || {
  echo "template-atlas must contain a committed revision: $template_root" >&2
  exit 1
}

mkdir -p "$atlas_root/target"
template_work="$(mktemp -d "$atlas_root/target/template-consumer-work.XXXXXX")"
cleanup() {
  case "$template_work" in
    "$atlas_root"/target/template-consumer-work.*)
      rm -rf -- "$template_work"
      ;;
    *)
      echo "Refusing to remove unexpected temporary path: $template_work" >&2
      ;;
  esac
}
trap cleanup EXIT

git -C "$template_root" archive --format=tar HEAD | tar -xf - -C "$template_work"
mkdir -p "$template_work/.cargo"
{
  printf '%s\n' '[patch.crates-io]'
  printf 'atlas-ui = { path = "%s" }\n' "$atlas_root/crates/atlas-ui"
} > "$template_work/.cargo/config.toml"

template_root="$template_work"

products="command forge fleet ledger"
check_target="$atlas_root/target/template-consumer-gate"
capture_root="$atlas_root/target/template-consumer-captures"

for product in $products; do
  manifest="$template_root/slint/$product/Cargo.toml"
  [ -f "$manifest" ] || {
    echo "Template manifest not found: $manifest" >&2
    exit 1
  }
  (
    cd "$template_root"
    CARGO_TARGET_DIR="$check_target" cargo check \
      --manifest-path "$manifest" --all-targets
  )
done

echo "Template consumer compilation passed: Command, Forge, Fleet, and Ledger."

if [ "$capture" -eq 1 ]; then
  mkdir -p "$capture_root"
  total=0
  for product in $products; do
    capture_script="$template_root/scripts/capture-$product-native.sh"
    [ -x "$capture_script" ] || {
      echo "Template capture script is missing or not executable: $capture_script" >&2
      exit 1
    }
    (
      cd "$template_root"
      "$capture_script" "$capture_root/$product"
    )
    case "$product" in
      command) expected=16 ;;
      forge) expected=26 ;;
      fleet) expected=30 ;;
      ledger) expected=25 ;;
    esac
    actual="$(find "$capture_root/$product" -type f -name '*.png' | wc -l | tr -d ' ')"
    [ "$actual" -eq "$expected" ] || {
      echo "Unexpected $product capture count: expected $expected, found $actual" >&2
      exit 1
    }
    total=$((total + actual))
  done
  echo "External consumer capture passed: $total states written to $capture_root."
fi
