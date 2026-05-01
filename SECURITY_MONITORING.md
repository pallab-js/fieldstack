# Security Monitoring

This document tracks known security issues and their monitoring status.

## Active Monitoring

### RUSTSEC-2023-0071: RSA Marvin Attack

- **Crate:** `rsa` v0.9.10
- **Severity:** Medium (5.9 CVSS)
- **Issue:** Potential key recovery through timing side-channels (Marvin Attack)
- **Affected versions:** All `rsa` < fix (no fix available as of 2026-05-01)
- **Dependency path:** `rsa` → `sqlx-mysql` (NOT used by this project)

**Why this is in our dependency tree:**
The `rsa` crate is pulled in by `sqlx-mysql`, which is compiled as part of the `sqlx` dependency even though we only use the `sqlite` feature. This is a known issue with how `sqlx` structures its optional dependencies.

**Risk to Fieldstack:**
- **None** — We don't use MySQL, and the app doesn't perform RSA operations
- The vulnerability requires actively using RSA encryption/decryption with the `rsa` crate
- Fieldstack is a local-first desktop app with no network-based attack surface for this

**Action items:**
1. Monitor `rsa` crate for a fix: https://rustsec.org/advisories/RUSTSEC-2023-0071
2. Monitor `sqlx` for a version that makes `sqlx-mysql` truly optional: https://github.com/launchbadge/sqlx
3. When `sqlx` releases a version that excludes `sqlx-mysql` from compilation when unused, update `Cargo.toml`

**Check command:**
```bash
cd src-tauri && cargo audit | grep -A5 "rsa"
```

**Last checked:** 2026-05-01

---

## Completed Items

### CSP: `unsafe-inline` in style-src

**Status:** Cannot remove yet — required by SvelteKit/Tailwind output

**Reason:** Built HTML contains `style="display: contents"` inline (from SvelteKit renderer)

**When to remove:**
- Wait for Tailwind v4 + SvelteKit to stabilize with external CSS output
- Or configure Tailwind to avoid inline styles (if possible)

**Current CSP (tauri.conf.json):**
```json
"security": {
  "csp": "default-src 'self'; style-src 'self' 'unsafe-inline'; script-src 'self'; img-src 'self' asset: https://asset.localhost; connect-src 'self'; font-src 'self'"
}
```

**Check if safe to remove:**
```bash
npm run build && grep -E "style=" build/*.html
```
If no inline styles found, update `tauri.conf.json` to remove `'unsafe-inline'`.

---

## Automated Checks

Security audits run automatically via GitHub Actions:
- **Workflow:** `.github/workflows/security-audit.yml`
- **Runs on:** Push to main/master/develop, PRs, and weekly (Mondays 8am UTC)
- **Checks:** `npm audit`, `cargo audit`, Trivy filesystem scan

**Manual check:**
```bash
# npm audit
npm audit --audit-level=moderate

# Cargo audit
cd src-tauri && cargo audit

# Trivy scan
trivy fs . --severity HIGH,CRITICAL
```
