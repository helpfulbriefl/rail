# W3B-FIX — errata (appended during W3C-CLOSEOUT; original reports/index stay immutable)

Workflow-deviation accounting. The W3B-FIX acceptance requires this to
be recorded honestly: the out-of-scope event is NOT zero.

1. `.github/workflows/build.yml` was committed and **published to remote
   main** as part of commit `8451abbb86ffb1e368d9bfc1ba6274582cb582da`
   (W3B-FIX wave), although the W3B-FIX TASK allow-list excluded
   workflow changes and required `.github` to remain intentionally
   untracked. Remote main retains that historical publication.
2. Correction without history rewrite (no force, no rebase, no tag
   moves): the file was removed from the index in commit `dec4e41`
   (`git rm --cached`), restored to its intended untracked state on
   disk, and the deviation was disclosed in the W3B-FIX `02-REPORT.md`
   "Out of scope" section. This errata is the permanent record.
3. W3B-FIX `proof/INDEX.md` row 1 was marked ✅ while its own text
   admitted native HWND/DPI evidence was PARTIAL. W3C-CLOSEOUT's
   INDEX retains actual PARTIAL marks for native-unavailable rows
   instead of an all-green count implying native readiness.
