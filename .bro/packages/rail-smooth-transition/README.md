# Rail smooth-transition — analyst handoff

This branch publishes a package only. Do NOT merge or switch to this branch in the coder working tree. `main` and application sources were not changed by this delivery.

- Exact ZIP size: 39964 bytes.
- ZIP SHA256: `ee478bc8e37d2475c4e85c678a3c085a72e47ff046b50410fdaf4bd857e34df1`.
- `rail-smooth-transition.zip.b64`: lossless Base64 encoding of the original binary ZIP (the publishing tool accepts text files only).
- `Get-Package.ps1`: downloads, decodes, verifies SHA256 and extracts OUTSIDE the project by default, without applying anything.

Run the helper from a saved script file; it restores the exact original ZIP automatically. Then follow `CODER-TASK.md` inside the package. Apply only AFTER the previous hover-stability patch; canonical CRLF/LF verification is included. No reset, merge, checkout, pull or auth changes are required to retrieve this package.

Publication of this handoff is Owner-authorized. It does NOT authorize publishing coder product changes, changing scope or declaring native Windows PASS.
