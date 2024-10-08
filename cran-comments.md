## R CMD check results

This submission resolves WARNS on CRAN checks.

- It adds `tests/` to `.Rbuildignore`.
- It adds files in `inst/` to `.Rbuildignore` to remove installed size NOTE

0 errors | 0 warnings | 0 note

## Check environments:

- macos-latest, r: release, rust: nightly
- windows-latest, r: release, rust: nightly
- ubuntu-latest, r: devel, rust: nightly
- ubuntu-latest, r: release, rust: nightly
- ubuntu-latest, r: oldrel-1, rust: nightly
- fedora 38, r: release, rust: 1.78
- fedora 38, r: release, rust: 1.67 (fails)

## Notes

* Minimum supported rust version (MSRV) is set to 1.70 this **will fail** gracefully on CRAN's Fedora machine.
    * this is checked by tools/msrv.R via `configure` and `configure.win`


## Tarball size

Tarball is 4.7mb due to vendored rust depencies. 
 