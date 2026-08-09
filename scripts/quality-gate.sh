#!/usr/bin/env sh
set -eu

atlas_cargo_bin="${CARGO_BIN:-cargo}"

"$atlas_cargo_bin" fmt --all -- --check
"$atlas_cargo_bin" check --workspace --all-targets
"$atlas_cargo_bin" clippy --workspace --all-targets -- -D warnings
"$atlas_cargo_bin" test --workspace --all-targets
node scripts/validate-agent-kit.mjs
node scripts/validate-agent-evals.mjs
node scripts/generate-agent-manifest.mjs --check
node scripts/validate-publication.mjs
if [ -d ai ]; then
    node ai/validate_ai.mjs
    node ai/validate_slint_compatibility.mjs
    node ai/validate_tokens.mjs
    node ai/validate_visual_scenarios.mjs
    node ai/validate_public_api.mjs
    node ai/validate_contrast.mjs
    node ai/validate_accessibility.mjs
    node ai/validate_performance_budgets.mjs
    node ai/validate_render_performance.mjs
    node ai/validate_component_compatibility.mjs
    node ai/validate_api_surface_audit.mjs
    node ai/validate_release_readiness.mjs
    node ai/validate_icons.mjs
    node ai/validate_fonts.mjs
fi
node scripts/capture-scenarios.mjs --validate-only
if [ -d ai ]; then
    node ai/validate_retirement.mjs
    node ai/audit_legacy.mjs >/dev/null
fi
