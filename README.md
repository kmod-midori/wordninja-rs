# Word Ninja
A Rust port of [wordninja](https://github.com/keredson/wordninja) which passes all the tests and is significantly faster.

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]
[![AUR][aur-badge]][aur-url]

[docs-badge]: https://docs.rs/wordninja/badge.svg
[docs-url]: https://docs.rs/wordninja
[crates-badge]: https://img.shields.io/crates/v/wordninja.svg
[crates-url]: https://crates.io/crates/wordninja
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE
[aur-badge]: https://img.shields.io/aur/version/wordninja-rs.svg
[aur-url]: https://aur.archlinux.org/packages/wordninja-rs

## Performance
Splitting the string
```
wethepeopleoftheunitedstatesinordertoformamoreperfectunionestablishjusticeinsuredomestictranquilityprovideforthecommondefencepromotethegeneralwelfareandsecuretheblessingsoflibertytoourselvesandourposteritydoordainandestablishthisconstitutionfortheunitedstatesofamerica
```

```
Benchmark #1: python t.py                                                                                                                                                                   0
  Time (mean ± σ):     188.9 ms ±   4.6 ms    [User: 0.9 ms, System: 5.5 ms]                                                                                                             0
  Range (min … max):   180.5 ms … 197.6 ms    15 runs

Benchmark #2: E:\Workspace\wordninja\target\release\wordninja.exe wethepeopleoftheunitedstatesinordertoformamoreperfectunionestablishjusticeinsuredomestictranquilityprovideforthecommondefencepromotethegeneralwelfareandsecuretheblessingsoflibertytoourselvesandourposteritydoordainandestablishthisconstitutionfortheunitedstatesofamerica
  Time (mean ± σ):      25.9 ms ±   1.0 ms    [User: 2.4 ms, System: 4.5 ms]                                                                                                             0
  Range (min … max):    24.1 ms …  29.8 ms    92 runs

Summary
  'E:\Workspace\wordninja\target\release\wordninja.exe wethepeopleoftheunitedstatesinordertoformamoreperfectunionestablishjusticeinsuredomestictranquilityprovideforthecommondefencepromotethegeneralwelfareandsecuretheblessingsoflibertytoourselvesandourposteritydoordainandestablishthisconstitutionfortheunitedstatesofamerica' ran
    7.31 ± 0.32 times faster than 'python t.py'
```
