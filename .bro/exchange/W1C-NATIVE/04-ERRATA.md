# 04-ERRATA — W1C-NATIVE (additive; S3-AUTH owner clarification; REPORT/SECURITY kept immutable)

Scope note: this file **adds** an S3-AUTH approval/erratum to the W1C-NATIVE
wave only. It does **not** rewrite or modify the existing `02-REPORT.md`
(passport still reads `push: ❌`) nor any security documentation. It is a
purely additive record. Precedent for this format: `../W1B-BASELINE/04-ERRATA.md`.

## E-007 — S3-AUTH: re-verify push access via command-local GCM (owner clarification)

**Authorization (owner).** The owner allows checking **write access** to
`helpfulbriefl/rail` through the **already-installed Git Credential Manager**
(recon: GCM `2.7.3+5fa71168`) and the **existing `helpfulbriefl`
credentials**. This **replaces** the previously-used route that involved
switching to / using the `Apex4Coder` gh account (cf. `diag/push-clean.bat`:
`gh auth switch -u helpfulbriefl` → `gh auth token` → push URL with embedded
token → `gh auth switch -u Apex4Coder`). That account-switching +
token-in-URL route is now **DISALLOWED**.

**Constraints (command-local GCM only).** The check runs with command-local
GCM overrides and MUST **not**:
- switch the `gh` account;
- retrieve or print a token (no `gh auth token`, no token echo);
- use `git credential fill`;
- embed credentials in the URL (no `https://user:token@host/...`);
- change the global `credential.helper`, the `remote`, or stored credentials;
- launch a new authentication flow automatically;
- on **error: STOP** and report the error without secrets — no alternative
  workaround is attempted.

**Command executed (owner's PowerShell block, verbatim `authArgs`).**
```powershell
Set-Location 'C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish'
git credential-manager --version   # throw 'Git Credential Manager недоступен. STOP.' if non-zero
# save/restore GIT_TERMINAL_PROMPT & GCM_INTERACTIVE; inside try set '0'/'Never'
$authArgs = @('-c','credential.helper=',
              '-c','credential.helper=manager',
              '-c','credential.https://github.com.username=helpfulbriefl')
git @authArgs push --dry-run --porcelain 'https://github.com/helpfulbriefl/rail.git' 'HEAD:refs/heads/main'
# throw 'Проверка push не прошла. STOP: сообщить ошибку без секретов.' if non-zero
```
`push --dry-run --porcelain` authenticates (proving write access on the
private repo) **without** mutating the remote.

**Pre-run ground truth (read-only recon, no auth).** GCM `2.7.3`, exit 0 ·
HEAD `562c3c58509dfa59c680e266af5c6c17f4d7d9c5` on `main` · origin
`https://github.com/helpfulbriefl/rail` · tag `w1c-native-done` →
`562c3c58509dfa59c680e266af5c6c17f4d7d9c5`.

**Result (command-local GCM dry-run, owner's exact `authArgs`, non-interactive).**

- `git credential-manager --version` → `2.7.3+5fa71168…`, exit 0.
- Dry-run **exit code = 1**. Full output (secrets-free):
  - `error: failed to push some refs to 'https://github.com/helpfulbriefl/rail.git'`
  - `hint: Updates were rejected because the remote contains work that you do
    not have locally … use 'git pull' before pushing again.`
  - `!	HEAD:refs/heads/main	[rejected] (fetch first)`
- **Access verdict:** AUTHENTICATION **PASSED**. The rejection is git's
  client-side non-fast-forward guard (`fetch first`), which only fires after a
  successful ref advertisement — i.e. the command-local GCM (`manager` +
  `credential.https://github.com.username=helpfulbriefl`) **did** supply a
  working `helpfulbriefl` credential and GitHub accepted it (no 401/403).
  Hence **write access via the already-installed GCM is confirmed**, which is
  precisely what S3-AUTH authorized.
- **Ref-divergence flag (analyst):** remote `main` is **ahead of / diverged
  from** local `main` (`562c3c58509d…`). The earlier "remote==local" claim no
  longer holds; something advanced remote `main` after the W1C push.
- **Per owner rule: STOP, no workaround.** No `git pull` / rebase / force was
  run; no account switch, no token retrieval/print, no `credential fill`, no
  credentials-in-URL, no global-helper/remote/stored-credential change, no
  auto-launched auth.

> Note: this file is additive. `02-REPORT.md` passport still reads
> `push: ❌` and is intentionally **not** rewritten; the `push` line there
> reflects the original wave delivery, while S3-AUTH is a separate
> access-verification erratum.
