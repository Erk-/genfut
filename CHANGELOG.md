# Changelog

## 0.4.0

- Implement std::error::Error for FutharkError and Error ([#26](https://github.com/Erk-/genfut/pull/26))
- Quote braces in static source. ([#25](https://github.com/Erk-/genfut/pull/25))
- Add error checking when creating the Context ([#24](https://github.com/Erk-/genfut/pull/24))
- Panic if freeing fails. ([#21](https://github.com/Erk-/genfut/pull/21))
- Fix values error ([#22](https://github.com/Erk-/genfut/pull/22))

## 0.3.0

- Don't compile OpenCL on MacOS (for now). ([#14](https://github.com/Erk-/genfut/pull/14))
- FEAT: Added flag -std=c99 to enable building on older versions of gcc (4.8.5 specifically) ([#13](https://github.com/Erk-/genfut/pull/13))

## 0.2.1

- Include Futhark version in generated output ([#12](https://github.com/Erk-/genfut/pull/12))
- Sync context after getting values ([#11](https://github.com/Erk-/genfut/pull/11))

## 0.2.0

- Fix docs and test ([#10](https://github.com/Erk-/genfut/pull/10))
- Do not complain if the desired directory already exists. ([#9](https://github.com/Erk-/genfut/pull/9))
- Futhark since 0.15.7 now returns const shapes for arrays. ([#8](https://github.com/Erk-/genfut/pull/8))

## 0.1.6

- Remove unused imports.
- Expose context fields.
([#7](https://github.com/Erk-/genfut/pull/7))

## 0.1.5

- Implement Sync and Send for FutharkContex.
- Never run bindgen at build time.
- Make bindings module public.
([#6](https://github.com/Erk-/genfut/pull/6))

## 0.1.4

Supply more Cargo fields, and don't break when building OpenCL lib on MacOS ([#5](https://github.com/Erk-/genfut/pull/5))

## 0.1.3

Remove unused import. ([#4](https://github.com/Erk-/genfut/pull/4))

Tweak library build ([#3](https://github.com/Erk-/genfut/pull/3))
Also changed opaque types to be taken by reference.


## 0.1.2
Remove comma from generation of opaque types ([#2](https://github.com/Erk-/genfut/pull/2#start-of-content))

