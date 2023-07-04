# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.8.3 (2023-07-04)

### Chore

 - <csr-id-5302905dbf6b9f2dbe64c8886f6245c45df4d5bf/> Update all deps

### Refactor

 - <csr-id-56c4c6998f6e6cab58492effbe44ae8b84ca4d03/> No need to 3 argument in InstallPtogress::Pending
 - <csr-id-e493db75fdf0e6edb287861dd25ee4a629f200b0/> Improve frontend error handle

### Style

 - <csr-id-d3acdffa3494b425cb1e1fb877a79c770dc4057a/> Use cargo-fmt to format code

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 4 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Use cargo-fmt to format code ([`d3acdff`](https://github.com/AOSC-Dev/aoscdk-rs/commit/d3acdffa3494b425cb1e1fb877a79c770dc4057a))
    - Update all deps ([`5302905`](https://github.com/AOSC-Dev/aoscdk-rs/commit/5302905dbf6b9f2dbe64c8886f6245c45df4d5bf))
    - No need to 3 argument in InstallPtogress::Pending ([`56c4c69`](https://github.com/AOSC-Dev/aoscdk-rs/commit/56c4c6998f6e6cab58492effbe44ae8b84ca4d03))
    - Improve frontend error handle ([`e493db7`](https://github.com/AOSC-Dev/aoscdk-rs/commit/e493db75fdf0e6edb287861dd25ee4a629f200b0))
</details>

## v0.8.2 (2023-06-29)

### Bug Fixes

 - <csr-id-6a78f7109bf62a16d0b2ce443fa4cf8165bd80a0/> Remove ui string useless text

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump aoscdk-rs v0.8.2 ([`144a639`](https://github.com/AOSC-Dev/aoscdk-rs/commit/144a639bab98a46298d0f8ef6c658449eb224a03))
    - Remove ui string useless text ([`6a78f71`](https://github.com/AOSC-Dev/aoscdk-rs/commit/6a78f7109bf62a16d0b2ce443fa4cf8165bd80a0))
</details>

## v0.8.1 (2023-06-29)

### New Features

 - <csr-id-399d19a0988bb3117fcea335b4058d9a8e97decd/> Improve ui string

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump aoscdk-rs v0.8.1 ([`c92376b`](https://github.com/AOSC-Dev/aoscdk-rs/commit/c92376b336db85e75920282cfde9161f151da719))
    - Improve ui string ([`399d19a`](https://github.com/AOSC-Dev/aoscdk-rs/commit/399d19a0988bb3117fcea335b4058d9a8e97decd))
    - Revert "feat: improve ui string" ([`0cb5447`](https://github.com/AOSC-Dev/aoscdk-rs/commit/0cb5447f33fc17935a454b0eff3e9da6704ecaed))
</details>

## v0.8.0 (2023-06-29)

<csr-id-89c3438c87140d20cda364706d0078749c00429f/>
<csr-id-66ff0f1082bd5ce47e99ae457e91bc5bb367f21b/>
<csr-id-b1acdb5aed3d2c7196bda22a289bee1bcaa77341/>
<csr-id-b03e080588fadbbe7ba69ae2e2338cb9adbafb47/>
<csr-id-2c39c88a4edaf7dfd6261b6b253740c89b7c7509/>

### Chore

 - <csr-id-89c3438c87140d20cda364706d0078749c00429f/> Update all deps
 - <csr-id-66ff0f1082bd5ce47e99ae457e91bc5bb367f21b/> Use AGPLv3 LICENSE
   sudoku use AGPLv3 license...so, AGPLv3.

### New Features

<csr-id-576cb12cd03c6aa58cc293b6e9324f74c0285863/>

 - <csr-id-2bc3dee568862fa1febf7cafcc46c467ac4d4d63/> Improve ui string
 - <csr-id-40d7ce1c3c0f96dd6be3238d0ca4048c925a20c0/> Improve speed display
   - Also add download eta display

### Bug Fixes

 - <csr-id-3a1e86bc65fe387c8666a843e65f719de3262c50/> Fix velocity calc
 - <csr-id-219102d2ae0258e8164b414a395bdaf0df430c0c/> Handle if reader_size / 0
 - <csr-id-b692fd7f950bf192b3248f7ffec1c9fce46756ea/> Velocity display issue

### Refactor

 - <csr-id-b1acdb5aed3d2c7196bda22a289bee1bcaa77341/> Improve download error handle
 - <csr-id-b03e080588fadbbe7ba69ae2e2338cb9adbafb47/> Use async reqwest to download squashfs

### Style

 - <csr-id-2c39c88a4edaf7dfd6261b6b253740c89b7c7509/> Use cargo-fmt and cargo-clippy to lint code

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 4 calendar days.
 - 5 days passed between releases.
 - 11 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump aoscdk-rs v0.8.0 ([`4f64757`](https://github.com/AOSC-Dev/aoscdk-rs/commit/4f6475701fe74c10d28a132b4c2a4f4628f966c4))
    - Update all deps ([`89c3438`](https://github.com/AOSC-Dev/aoscdk-rs/commit/89c3438c87140d20cda364706d0078749c00429f))
    - Improve ui string ([`2bc3dee`](https://github.com/AOSC-Dev/aoscdk-rs/commit/2bc3dee568862fa1febf7cafcc46c467ac4d4d63))
    - Use cargo-fmt and cargo-clippy to lint code ([`2c39c88`](https://github.com/AOSC-Dev/aoscdk-rs/commit/2c39c88a4edaf7dfd6261b6b253740c89b7c7509))
    - Improve speed display ([`40d7ce1`](https://github.com/AOSC-Dev/aoscdk-rs/commit/40d7ce1c3c0f96dd6be3238d0ca4048c925a20c0))
    - Improve download error handle ([`b1acdb5`](https://github.com/AOSC-Dev/aoscdk-rs/commit/b1acdb5aed3d2c7196bda22a289bee1bcaa77341))
    - Use async reqwest to download squashfs ([`b03e080`](https://github.com/AOSC-Dev/aoscdk-rs/commit/b03e080588fadbbe7ba69ae2e2338cb9adbafb47))
    - Fix velocity calc ([`3a1e86b`](https://github.com/AOSC-Dev/aoscdk-rs/commit/3a1e86bc65fe387c8666a843e65f719de3262c50))
    - Handle if reader_size / 0 ([`219102d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/219102d2ae0258e8164b414a395bdaf0df430c0c))
    - Velocity display issue ([`b692fd7`](https://github.com/AOSC-Dev/aoscdk-rs/commit/b692fd7f950bf192b3248f7ffec1c9fce46756ea))
    - Add velocity display in installing view ([`576cb12`](https://github.com/AOSC-Dev/aoscdk-rs/commit/576cb12cd03c6aa58cc293b6e9324f74c0285863))
    - Use AGPLv3 LICENSE ([`66ff0f1`](https://github.com/AOSC-Dev/aoscdk-rs/commit/66ff0f1082bd5ce47e99ae457e91bc5bb367f21b))
</details>

## v0.7.4 (2023-06-24)

### New Features

 - <csr-id-1d519034b0acfdaa9925cc5e14f982c4c115106c/> Add nofail option to fstab esp entry
 - <csr-id-b9ee09f748e38358af75a6b3cb1b332daa2748aa/> Do not write esp partition entry to fstab

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump aoscdk-rs v0.7.4 ([`aa14200`](https://github.com/AOSC-Dev/aoscdk-rs/commit/aa14200117116914ba847a0feaacee26e3d8ef75))
    - Add nofail option to fstab esp entry ([`1d51903`](https://github.com/AOSC-Dev/aoscdk-rs/commit/1d519034b0acfdaa9925cc5e14f982c4c115106c))
    - Revert "feat: do not write esp partition entry to fstab" ([`9f5844a`](https://github.com/AOSC-Dev/aoscdk-rs/commit/9f5844af428a4aa8d269df8f8befc52ec265c68a))
    - Do not write esp partition entry to fstab ([`b9ee09f`](https://github.com/AOSC-Dev/aoscdk-rs/commit/b9ee09f748e38358af75a6b3cb1b332daa2748aa))
</details>

## v0.7.3 (2023-06-24)

### Bug Fixes

 - <csr-id-af19c0f97323360e584362d5b573a7c4bf928ae9/> Do not set fullname if user setting fullname is empty
 - <csr-id-cae9b397a968f42c3bc44d9ead835e336c37aec4/> Do not write swapfile entry to fstab if use_swap is false

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump aoscdk-rs v0.7.3 ([`dadac73`](https://github.com/AOSC-Dev/aoscdk-rs/commit/dadac7358602dc2b48ab12983ad501fca6a57f6a))
    - Do not set fullname if user setting fullname is empty ([`af19c0f`](https://github.com/AOSC-Dev/aoscdk-rs/commit/af19c0f97323360e584362d5b573a7c4bf928ae9))
    - Do not write swapfile entry to fstab if use_swap is false ([`cae9b39`](https://github.com/AOSC-Dev/aoscdk-rs/commit/cae9b397a968f42c3bc44d9ead835e336c37aec4))
</details>

## v0.7.2 (2023-06-24)

<csr-id-90f448d4ce42b04cc351cb810b04797f6982b58e/>
<csr-id-e7d8227e92f7a363ff7dc0d55a81d31b898b8b8f/>
<csr-id-70c89b624e5836b96022e170e422920db83a19c6/>

### Chore

 - <csr-id-90f448d4ce42b04cc351cb810b04797f6982b58e/> Update all deps

### New Features

 - <csr-id-09b4e6439a3da30136ceb633657aa0b227e375e9/> Only allow ext4 and xfs as system disk type

### Bug Fixes

 - <csr-id-7ab5d1a2f0f3662b9d146878745986b43d2fb066/> 'Use Ext4' after shuld pop layer window
   - Also set 'Use ext4' button as default option

### Refactor

 - <csr-id-e7d8227e92f7a363ff7dc0d55a81d31b898b8b8f/> Use run_command function to run systemctl reboot

### Style

 - <csr-id-70c89b624e5836b96022e170e422920db83a19c6/> Use cargo-fmt and cargo-clippy to lint code

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 1 day passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump aoscdk-rs v0.7.2 ([`34f6d53`](https://github.com/AOSC-Dev/aoscdk-rs/commit/34f6d53aefd761c3f79d19002fb468ea58f0a66e))
    - Use cargo-fmt and cargo-clippy to lint code ([`70c89b6`](https://github.com/AOSC-Dev/aoscdk-rs/commit/70c89b624e5836b96022e170e422920db83a19c6))
    - Update all deps ([`90f448d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/90f448d4ce42b04cc351cb810b04797f6982b58e))
    - Only allow ext4 and xfs as system disk type ([`09b4e64`](https://github.com/AOSC-Dev/aoscdk-rs/commit/09b4e6439a3da30136ceb633657aa0b227e375e9))
    - 'Use Ext4' after shuld pop layer window ([`7ab5d1a`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7ab5d1a2f0f3662b9d146878745986b43d2fb066))
    - Use run_command function to run systemctl reboot ([`e7d8227`](https://github.com/AOSC-Dev/aoscdk-rs/commit/e7d8227e92f7a363ff7dc0d55a81d31b898b8b8f))
</details>

## v0.7.1 (2023-06-22)

<csr-id-8f8881f3971261905ec2532ec6db7587b58c2dff/>
<csr-id-1152a702f5a01d914db765af7fca0dfb83bd7dec/>

### Chore

 - <csr-id-8f8881f3971261905ec2532ec6db7587b58c2dff/> Update all deps

### New Features

 - <csr-id-4b0275db4895bcd88313efdc7f2e229f3363acca/> Limit unsquashfs thread to 1 if total ram <= 2GiB

### Style

 - <csr-id-1152a702f5a01d914db765af7fca0dfb83bd7dec/> Use cargo-fmt and cargo-clippy to lint code

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 4 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump aoscdk-rs v0.7.1 ([`0293f4d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/0293f4db8342c9de87953837808000cdd2681e19))
    - Use cargo-fmt and cargo-clippy to lint code ([`1152a70`](https://github.com/AOSC-Dev/aoscdk-rs/commit/1152a702f5a01d914db765af7fca0dfb83bd7dec))
    - Update all deps ([`8f8881f`](https://github.com/AOSC-Dev/aoscdk-rs/commit/8f8881f3971261905ec2532ec6db7587b58c2dff))
    - Limit unsquashfs thread to 1 if total ram <= 2GiB ([`4b0275d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/4b0275db4895bcd88313efdc7f2e229f3363acca))
</details>

## v0.7.0 (2023-06-18)

<csr-id-bfae4800da3880bd2a156b444c0334460f03952b/>
<csr-id-ff8f76a3ccd9c15d431c321567a2d340bb80fc1b/>
<csr-id-b1c66da54dd953bd619163b95c12ab1cafdc4663/>
<csr-id-4f1196630a8f12352317109e866d40c095aa3730/>
<csr-id-7dce115764e6f2abf6e4f1eebb318456bc8d1352/>

### Chore

 - <csr-id-bfae4800da3880bd2a156b444c0334460f03952b/> Use cargo-fmt to format code
 - <csr-id-ff8f76a3ccd9c15d431c321567a2d340bb80fc1b/> Update all deps

### New Features

 - <csr-id-d94587836076f0e1b0fc28154c3cfeba49900626/> Only display squashfs option in download list
 - <csr-id-0830473739b8f0f7bed92c61e2c079c26ca3af2f/> Add squashfs option to variant list

### Other

 - <csr-id-b1c66da54dd953bd619163b95c12ab1cafdc4663/> Update dependencies
 - <csr-id-4f1196630a8f12352317109e866d40c095aa3730/> Add extract squashfs support

### Refactor

 - <csr-id-7dce115764e6f2abf6e4f1eebb318456bc8d1352/> Drop tarball file after download done

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 7 calendar days.
 - 22 days passed between releases.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump aoscdk-rs v0.7.0 ([`ffe89c3`](https://github.com/AOSC-Dev/aoscdk-rs/commit/ffe89c33a7effd036b61ab049624e2213f2ca05d))
    - Use cargo-fmt to format code ([`bfae480`](https://github.com/AOSC-Dev/aoscdk-rs/commit/bfae4800da3880bd2a156b444c0334460f03952b))
    - Update all deps ([`ff8f76a`](https://github.com/AOSC-Dev/aoscdk-rs/commit/ff8f76a3ccd9c15d431c321567a2d340bb80fc1b))
    - Only display squashfs option in download list ([`d945878`](https://github.com/AOSC-Dev/aoscdk-rs/commit/d94587836076f0e1b0fc28154c3cfeba49900626))
    - Drop tarball file after download done ([`7dce115`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7dce115764e6f2abf6e4f1eebb318456bc8d1352))
    - Add squashfs option to variant list ([`0830473`](https://github.com/AOSC-Dev/aoscdk-rs/commit/0830473739b8f0f7bed92c61e2c079c26ca3af2f))
    - Update dependencies ([`b1c66da`](https://github.com/AOSC-Dev/aoscdk-rs/commit/b1c66da54dd953bd619163b95c12ab1cafdc4663))
    - Add extract squashfs support ([`4f11966`](https://github.com/AOSC-Dev/aoscdk-rs/commit/4f1196630a8f12352317109e866d40c095aa3730))
</details>

## v0.6.3 (2023-05-27)

<csr-id-d7a9dedc98fd9f815b673eb5e25ea5f0dede879c/>
<csr-id-210a679d076a058bdf8456528df3ac9d8e274a69/>
<csr-id-439bd899f17fe8398df3ead37099f43dcfbf2ee6/>
<csr-id-329d420ea48c90c630e04d0d88afea3251008845/>

### Chore

 - <csr-id-d7a9dedc98fd9f815b673eb5e25ea5f0dede879c/> Use cargo-fmt to format code
 - <csr-id-210a679d076a058bdf8456528df3ac9d8e274a69/> Update all deps
 - <csr-id-439bd899f17fe8398df3ead37099f43dcfbf2ee6/> Update all deps

### Bug Fixes

 - <csr-id-532b757a0cf4fd18354e767a5bd308ef85e7ccd3/> Do not set tui ctrlc handler
   This fix solves the problem that aoscdk-rs crashes when it is first installed
   
   Since the ctrlc-c binding has been cleared from the cursive level, there is no need to use ctrlc create to set the ctrlc handler

### Style

 - <csr-id-329d420ea48c90c630e04d0d88afea3251008845/> Use cargo-fmt to format code

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 5 calendar days.
 - 5 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump aoscdk-rs v0.6.3 ([`4ce7a87`](https://github.com/AOSC-Dev/aoscdk-rs/commit/4ce7a871635068eb5c3742c3fb39f6c664a52da0))
    - Use cargo-fmt to format code ([`d7a9ded`](https://github.com/AOSC-Dev/aoscdk-rs/commit/d7a9dedc98fd9f815b673eb5e25ea5f0dede879c))
    - Update all deps ([`210a679`](https://github.com/AOSC-Dev/aoscdk-rs/commit/210a679d076a058bdf8456528df3ac9d8e274a69))
    - Do not set tui ctrlc handler ([`532b757`](https://github.com/AOSC-Dev/aoscdk-rs/commit/532b757a0cf4fd18354e767a5bd308ef85e7ccd3))
    - Use cargo-fmt to format code ([`329d420`](https://github.com/AOSC-Dev/aoscdk-rs/commit/329d420ea48c90c630e04d0d88afea3251008845))
    - Update all deps ([`439bd89`](https://github.com/AOSC-Dev/aoscdk-rs/commit/439bd899f17fe8398df3ead37099f43dcfbf2ee6))
</details>

## v0.6.2 (2023-05-21)

### New Features

 - <csr-id-ef5e61ff674a37ef1fc6e090d0c3ffec40f4606d/> Use systemd reboot to reboot system
   - Also fix mirror benchmark

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 22 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump aoscdk-rs v0.6.2 ([`503911a`](https://github.com/AOSC-Dev/aoscdk-rs/commit/503911a66ea8b7f9ca6323840e6a46114d4520c3))
    - Use systemd reboot to reboot system ([`ef5e61f`](https://github.com/AOSC-Dev/aoscdk-rs/commit/ef5e61ff674a37ef1fc6e090d0c3ffec40f4606d))
</details>

## v0.6.1 (2023-04-29)

<csr-id-87db8ec5a7e87d27352167f26d78c26229d9a65d/>

### Chore

 - <csr-id-87db8ec5a7e87d27352167f26d78c26229d9a65d/> update all deps

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump aoscdk-rs v0.6.1 ([`f8cfcd8`](https://github.com/AOSC-Dev/aoscdk-rs/commit/f8cfcd8afc85b5bc101cb6660e8f347f5c8402e3))
    - Update all deps ([`87db8ec`](https://github.com/AOSC-Dev/aoscdk-rs/commit/87db8ec5a7e87d27352167f26d78c26229d9a65d))
    - Revert "fix: set termion backend to workaround numpad issue under KMSCON" ([`31e7160`](https://github.com/AOSC-Dev/aoscdk-rs/commit/31e716094a91af26b837fac3a473b4d24f39eaa4))
</details>

## v0.6.0 (2023-04-28)

<csr-id-6a539a850fb60ab3be499a16daa4414a652d1631/>
<csr-id-3f89b6a367fc70de21b70ef4d75ded6930188081/>

### Chore

 - <csr-id-6a539a850fb60ab3be499a16daa4414a652d1631/> use cargo-fmt to format code
 - <csr-id-3f89b6a367fc70de21b70ef4d75ded6930188081/> update all deps

### New Features

 - <csr-id-6b6e135b5fda6134b6c7f750b8f81569192af18b/> Allow user set useless swap size

### Bug Fixes

 - <csr-id-6fc6db1dd06fd14712cb204060c7e32b9b8c4f47/> Set termion backend to workaround numpad issue under KMSCON
 - <csr-id-49724d735fac2261ea88d1e731173466d89eaa36/> Mbr_is_primary_partition method need parent_path and part_path argument

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump aoscdk-rs v0.6.0 ([`04b7409`](https://github.com/AOSC-Dev/aoscdk-rs/commit/04b74093cca2c5632d082f48141a30b1f3f10af2))
    - Use cargo-fmt to format code ([`6a539a8`](https://github.com/AOSC-Dev/aoscdk-rs/commit/6a539a850fb60ab3be499a16daa4414a652d1631))
    - Set termion backend to workaround numpad issue under KMSCON ([`6fc6db1`](https://github.com/AOSC-Dev/aoscdk-rs/commit/6fc6db1dd06fd14712cb204060c7e32b9b8c4f47))
    - Mbr_is_primary_partition method need parent_path and part_path argument ([`49724d7`](https://github.com/AOSC-Dev/aoscdk-rs/commit/49724d735fac2261ea88d1e731173466d89eaa36))
    - Revert "fix: do not set parent_path as argument to mbr_is_primary_partition method" ([`125e3dd`](https://github.com/AOSC-Dev/aoscdk-rs/commit/125e3dd0393dec7282ba0300e7c0ad64ee726954))
    - Allow user set useless swap size ([`6b6e135`](https://github.com/AOSC-Dev/aoscdk-rs/commit/6b6e135b5fda6134b6c7f750b8f81569192af18b))
    - Fix frontend::cli::test Unsupported architecture issue ([`98f5096`](https://github.com/AOSC-Dev/aoscdk-rs/commit/98f50963293ac0cf5a7b49663d5b9fa184ab306f))
    - Update all deps ([`3f89b6a`](https://github.com/AOSC-Dev/aoscdk-rs/commit/3f89b6a367fc70de21b70ef4d75ded6930188081))
</details>

## v0.5.9 (2023-04-27)

<csr-id-01ee74b2518246c9dbad58d3cb4d665c8d5953f6/>
<csr-id-f0f333b98bd5053129107b6c115b12993b1ed7a9/>

### Chore

 - <csr-id-01ee74b2518246c9dbad58d3cb4d665c8d5953f6/> fix author info for Mag Mell (eatradish)

### Bug Fixes

 - <csr-id-63b3d81041b21eb0a77b96b6f07e5e8d77c79bce/> Do not set parent_path as argument to mbr_is_primary_partition method
 - <csr-id-9fc4227e0c37d3acc7bb4ef3434b7c3f639e6d41/> Avoid overlapping windows upon resumed setup on MBR drives
 - <csr-id-af4660e9d76a3b007ede3e6db3f1479deacbb400/> Set debug mode disk path as /dev/loop10 to fix conflict with snap
 - <csr-id-941fe8c0a02c4def9d8fd24cf9a882724df55c23/> Allow installing on !GPT partition tables on ppc64el
 - <csr-id-d303ec80310f2d2a4bdb38a82c1331903538d386/> Disable grub-install for PowerNV/OPAL systems
   These systems uses Petitboot/Skiboot, which picks up boot entries from GRUB
   configuration. No need for installed bootloaders.

### Style

 - <csr-id-f0f333b98bd5053129107b6c115b12993b1ed7a9/> use cargo clippy to lint code

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump aoscdk-rs v0.5.9 ([`01ce652`](https://github.com/AOSC-Dev/aoscdk-rs/commit/01ce6526d9ece1bba00000fb8bff23d838c3ac03))
    - Use cargo clippy to lint code ([`f0f333b`](https://github.com/AOSC-Dev/aoscdk-rs/commit/f0f333b98bd5053129107b6c115b12993b1ed7a9))
    - Fix author info for Mag Mell (eatradish) ([`01ee74b`](https://github.com/AOSC-Dev/aoscdk-rs/commit/01ee74b2518246c9dbad58d3cb4d665c8d5953f6))
    - Do not set parent_path as argument to mbr_is_primary_partition method ([`63b3d81`](https://github.com/AOSC-Dev/aoscdk-rs/commit/63b3d81041b21eb0a77b96b6f07e5e8d77c79bce))
    - Avoid overlapping windows upon resumed setup on MBR drives ([`9fc4227`](https://github.com/AOSC-Dev/aoscdk-rs/commit/9fc4227e0c37d3acc7bb4ef3434b7c3f639e6d41))
    - Set debug mode disk path as /dev/loop10 to fix conflict with snap ([`af4660e`](https://github.com/AOSC-Dev/aoscdk-rs/commit/af4660e9d76a3b007ede3e6db3f1479deacbb400))
    - Allow installing on !GPT partition tables on ppc64el ([`941fe8c`](https://github.com/AOSC-Dev/aoscdk-rs/commit/941fe8c0a02c4def9d8fd24cf9a882724df55c23))
    - Disable grub-install for PowerNV/OPAL systems ([`d303ec8`](https://github.com/AOSC-Dev/aoscdk-rs/commit/d303ec80310f2d2a4bdb38a82c1331903538d386))
</details>

## v0.5.8 (2023-04-26)

<csr-id-83aa175e9952cee96bf2afdb0c78b6bf2c3de844/>
<csr-id-7902d23512acaf4cf08c8ca48d23f6a71058b0a1/>
<csr-id-00c95bd617d4696bb13b753e691b3f47fa5db0f0/>

### Chore

 - <csr-id-83aa175e9952cee96bf2afdb0c78b6bf2c3de844/> update all deps
 - <csr-id-7902d23512acaf4cf08c8ca48d23f6a71058b0a1/> add Mag Mell as author

### New Features

 - <csr-id-e68b7328c6fb774bfd4d001431d34b85012ac5d4/> Add set fullname feature

### Bug Fixes

 - <csr-id-592acd0aa2db49204bba8534a6c3b57261368866/> (powerpc64) rename struct PathBuf => Path in src/disks.rs

### Style

 - <csr-id-00c95bd617d4696bb13b753e691b3f47fa5db0f0/> use cargo-fmt to format code

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 11 calendar days.
 - 15 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump aoscdk-rs v0.5.8 ([`748c0a3`](https://github.com/AOSC-Dev/aoscdk-rs/commit/748c0a394dc632b2be3fce1d894c55245a357684))
    - Use cargo-fmt to format code ([`00c95bd`](https://github.com/AOSC-Dev/aoscdk-rs/commit/00c95bd617d4696bb13b753e691b3f47fa5db0f0))
    - Update all deps ([`83aa175`](https://github.com/AOSC-Dev/aoscdk-rs/commit/83aa175e9952cee96bf2afdb0c78b6bf2c3de844))
    - Add Mag Mell as author ([`7902d23`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7902d23512acaf4cf08c8ca48d23f6a71058b0a1))
    - (powerpc64) rename struct PathBuf => Path in src/disks.rs ([`592acd0`](https://github.com/AOSC-Dev/aoscdk-rs/commit/592acd0aa2db49204bba8534a6c3b57261368866))
    - Add set fullname feature ([`e68b732`](https://github.com/AOSC-Dev/aoscdk-rs/commit/e68b7328c6fb774bfd4d001431d34b85012ac5d4))
</details>

## v0.5.7 (2023-04-12)

<csr-id-daa07b69856a8403b1bf11d387063c752dfaca38/>
<csr-id-95a17117f3e3d0576b16ea90d9556ffb12f68049/>

### Chore

 - <csr-id-daa07b69856a8403b1bf11d387063c752dfaca38/> update all deps

### New Features

 - <csr-id-1e387238bfe9f58bb9fa3a227192da3f8b22be7c/> Do not disable hibernate in deploykit

### Style

 - <csr-id-95a17117f3e3d0576b16ea90d9556ffb12f68049/> use cargo-fmt to format code

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 2 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump aoscdk-rs v0.5.7 ([`2e8b347`](https://github.com/AOSC-Dev/aoscdk-rs/commit/2e8b3471c676d8a91b312c8dcdcd30d83ecd6d40))
    - Use cargo-fmt to format code ([`95a1711`](https://github.com/AOSC-Dev/aoscdk-rs/commit/95a17117f3e3d0576b16ea90d9556ffb12f68049))
    - Update all deps ([`daa07b6`](https://github.com/AOSC-Dev/aoscdk-rs/commit/daa07b69856a8403b1bf11d387063c752dfaca38))
    - Do not disable hibernate in deploykit ([`1e38723`](https://github.com/AOSC-Dev/aoscdk-rs/commit/1e387238bfe9f58bb9fa3a227192da3f8b22be7c))
</details>

## v0.5.6 (2023-04-09)

<csr-id-4ac0590beae826e73ff0ed1da039d5ac7e9b10c9/>
<csr-id-f228a5c4782b8860a3faeea783f4d6f24225c63f/>
<csr-id-7eb7826e8eb9d1a003b38bf4400b76d4a86fab20/>
<csr-id-dfe24d99e8962de905fea50d93702fe7d93e4c8d/>
<csr-id-0dd8cc39e18d76a09da655124a858eb607700b32/>
<csr-id-2143fe8f9efc5693b6270689313f425de95a28ba/>
<csr-id-1f38cfb48a371cb413cca7ffb853c37b945d15df/>

### Chore

 - <csr-id-4ac0590beae826e73ff0ed1da039d5ac7e9b10c9/> remove -alpha.0 version tag; use rust 2021 edition
 - <csr-id-f228a5c4782b8860a3faeea783f4d6f24225c63f/> update all deps
 - <csr-id-7eb7826e8eb9d1a003b38bf4400b76d4a86fab20/> update all deps

### Documentation

 - <csr-id-9d540bf87c29a9d16e713ffb80da96974ad508c0/> Add changelog

### Bug Fixes

 - <csr-id-92a8ae90ace55f2a367131c3494ddaf1e63f5457/> Use libc::fallocate64 to fallocate swapfile to fix retro create swapfile
 - <csr-id-3ef14fc6f5f3e3b23853147d2c325daebd92129d/> Fix swapfile crate size

### Other

 - <csr-id-dfe24d99e8962de905fea50d93702fe7d93e4c8d/> check user mirror URL validity with a HEAD request
 - <csr-id-0dd8cc39e18d76a09da655124a858eb607700b32/> give mirror url input some padding
 - <csr-id-2143fe8f9efc5693b6270689313f425de95a28ba/> allow manual mirror URL selection
   A button "Specify URL" has been added. This allows a user to download
   from a specific mirror server - usually private or behind a firewall.
   
   Currently validity of the url is not enforced - DK should make a test
   GET request to the url to make sure the mirror is available.

### Style

 - <csr-id-1f38cfb48a371cb413cca7ffb853c37b945d15df/> use cargo clippy to lint code

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 48 calendar days.
 - 48 days passed between releases.
 - 10 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump aoscdk-rs v0.5.6 ([`75ac6bb`](https://github.com/AOSC-Dev/aoscdk-rs/commit/75ac6bb7e345ee1b6d989b81bf925bd9a0cd4b6e))
    - Add changelog ([`9d540bf`](https://github.com/AOSC-Dev/aoscdk-rs/commit/9d540bf87c29a9d16e713ffb80da96974ad508c0))
    - Remove -alpha.0 version tag; use rust 2021 edition ([`4ac0590`](https://github.com/AOSC-Dev/aoscdk-rs/commit/4ac0590beae826e73ff0ed1da039d5ac7e9b10c9))
    - Use cargo clippy to lint code ([`1f38cfb`](https://github.com/AOSC-Dev/aoscdk-rs/commit/1f38cfb48a371cb413cca7ffb853c37b945d15df))
    - Update all deps ([`f228a5c`](https://github.com/AOSC-Dev/aoscdk-rs/commit/f228a5c4782b8860a3faeea783f4d6f24225c63f))
    - Update all deps ([`7eb7826`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7eb7826e8eb9d1a003b38bf4400b76d4a86fab20))
    - Use libc::fallocate64 to fallocate swapfile to fix retro create swapfile ([`92a8ae9`](https://github.com/AOSC-Dev/aoscdk-rs/commit/92a8ae90ace55f2a367131c3494ddaf1e63f5457))
    - Fix swapfile crate size ([`3ef14fc`](https://github.com/AOSC-Dev/aoscdk-rs/commit/3ef14fc6f5f3e3b23853147d2c325daebd92129d))
    - Check user mirror URL validity with a HEAD request ([`dfe24d9`](https://github.com/AOSC-Dev/aoscdk-rs/commit/dfe24d99e8962de905fea50d93702fe7d93e4c8d))
    - Give mirror url input some padding ([`0dd8cc3`](https://github.com/AOSC-Dev/aoscdk-rs/commit/0dd8cc39e18d76a09da655124a858eb607700b32))
    - Allow manual mirror URL selection ([`2143fe8`](https://github.com/AOSC-Dev/aoscdk-rs/commit/2143fe8f9efc5693b6270689313f425de95a28ba))
    - (cargo-release) start next development iteration 0.5.6-alpha.0 ([`9cdbc67`](https://github.com/AOSC-Dev/aoscdk-rs/commit/9cdbc67ac5d32191ead51a03f93a272410bb8023))
</details>

## v0.5.5 (2023-02-20)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 18 calendar days.
 - 18 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.5 ([`534147c`](https://github.com/AOSC-Dev/aoscdk-rs/commit/534147c50b943369649298e40aa848b954f288f0))
    - Frontend{cli, tui}: fix disk space requirement calculation ([`40bbb25`](https://github.com/AOSC-Dev/aoscdk-rs/commit/40bbb2567d2fcb525d7a0552c778b5a92cd018c2))
    - (cargo-release) start next development iteration 0.5.5-alpha.0 ([`522caa0`](https://github.com/AOSC-Dev/aoscdk-rs/commit/522caa01775351b2bb0e27c0278753dee252ce8d))
</details>

## v0.5.4 (2023-02-01)

<csr-id-a0f396174d82eb1efe1fd5ef4c4ba60afb674e1a/>
<csr-id-b430bbc68fd6df8fba1dd24c9a4e70f09d95bedb/>
<csr-id-70d15a776cf5c6ea669928ad0463921a11f92dfc/>

### Other

 - <csr-id-a0f396174d82eb1efe1fd5ef4c4ba60afb674e1a/> use cargo clippy
 - <csr-id-b430bbc68fd6df8fba1dd24c9a4e70f09d95bedb/> fix username check like abc123
 - <csr-id-70d15a776cf5c6ea669928ad0463921a11f92dfc/> fix a typo 0-0 => 0-9

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 4 calendar days.
 - 4 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.4 ([`2edc084`](https://github.com/AOSC-Dev/aoscdk-rs/commit/2edc0844de5b86f77f203f7ff1d014ed7e1eb0fb))
    - Use cargo clippy ([`a0f3961`](https://github.com/AOSC-Dev/aoscdk-rs/commit/a0f396174d82eb1efe1fd5ef4c4ba60afb674e1a))
    - Fix username check like abc123 ([`b430bbc`](https://github.com/AOSC-Dev/aoscdk-rs/commit/b430bbc68fd6df8fba1dd24c9a4e70f09d95bedb))
    - Fix a typo 0-0 => 0-9 ([`70d15a7`](https://github.com/AOSC-Dev/aoscdk-rs/commit/70d15a776cf5c6ea669928ad0463921a11f92dfc))
    - (cargo-release) start next development iteration 0.5.4-alpha.0 ([`22c1a75`](https://github.com/AOSC-Dev/aoscdk-rs/commit/22c1a758da6fd1624af09e4866b4630baed9cd22))
</details>

## v0.5.3 (2023-01-28)

<csr-id-1ac38954474c4192f5c8fc0f01027d7a2fc78687/>
<csr-id-4bf359be2e85df5a95313cbec9a253dc19af5651/>

### Other

 - <csr-id-1ac38954474c4192f5c8fc0f01027d7a2fc78687/> adapt new sysinfo version
 - <csr-id-4bf359be2e85df5a95313cbec9a253dc19af5651/> cargo update; pin clap version as 4.0.32 to fix rustc 1.63.0 build

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.3 ([`84fd515`](https://github.com/AOSC-Dev/aoscdk-rs/commit/84fd515a47ef5efd9c6022bf66e7ff33f7f4ecac))
    - Adapt new sysinfo version ([`1ac3895`](https://github.com/AOSC-Dev/aoscdk-rs/commit/1ac38954474c4192f5c8fc0f01027d7a2fc78687))
    - Cargo update; pin clap version as 4.0.32 to fix rustc 1.63.0 build ([`4bf359b`](https://github.com/AOSC-Dev/aoscdk-rs/commit/4bf359be2e85df5a95313cbec9a253dc19af5651))
    - (cargo-release) start next development iteration 0.5.3-alpha.0 ([`bf4d185`](https://github.com/AOSC-Dev/aoscdk-rs/commit/bf4d185e0a94c323d92adda9699b11bcea6b1a37))
</details>

## v0.5.2 (2023-01-28)

<csr-id-6b37bc3f04f7390e2dbb02643a1102145c485db9/>
<csr-id-b34677268c80701e1a4a9be2f1b467748b415f3c/>

### Other

 - <csr-id-6b37bc3f04f7390e2dbb02643a1102145c485db9/> fix tips can't quit
 - <csr-id-b34677268c80701e1a4a9be2f1b467748b415f3c/> check if mbr partition type != primary, return error

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 100 calendar days.
 - 100 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.2 ([`6912a7d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/6912a7d491e24d02917ceee595e6b25abf1750a9))
    - Fix tips can't quit ([`6b37bc3`](https://github.com/AOSC-Dev/aoscdk-rs/commit/6b37bc3f04f7390e2dbb02643a1102145c485db9))
    - Check if mbr partition type != primary, return error ([`b346772`](https://github.com/AOSC-Dev/aoscdk-rs/commit/b34677268c80701e1a4a9be2f1b467748b415f3c))
    - (cargo-release) start next development iteration 0.5.2-alpha.0 ([`baaf5bc`](https://github.com/AOSC-Dev/aoscdk-rs/commit/baaf5bcf4a4c038d47942169246256e32b6be731))
</details>

## v0.5.1 (2022-10-19)

<csr-id-2ffbb5306034004223fc453ce5234415b2b056be/>
<csr-id-0f5f5bf67a735e7d7e25cc0be173f4fc75a0871b/>
<csr-id-daa407ebe2a1eb7c61ffd59f75080c3b058cf39b/>
<csr-id-2cb79f715be8285dc5742d1d4a93ad4330c8f8e5/>
<csr-id-7fb308c2a7b9f9084263a627045260621c386733/>
<csr-id-769f48882f6797debece8c5c7022a4b5c6aa326a/>
<csr-id-723332a1309a1f23209998a07a396540822f1a9e/>
<csr-id-952dd934ea7fa353513f969ff0452b51515d3281/>

### Other

 - <csr-id-2ffbb5306034004223fc453ce5234415b2b056be/> fix grub install to efi system in Linux 6.0 kernel
   In Linux 6.0, The obsolete 'efivars' sysfs-based interface has been removed.
   This change will cause the grub-install step in deploykit to fail.
   
   The current solution is to mount /sys/firmware/efi/efivars to
   /sys/firmware/efi/efivars where the root is installed when it detects that
   the user is using the EFI system.
 - <csr-id-0f5f5bf67a735e7d7e25cc0be173f4fc75a0871b/> fix cursive logger view
 - <csr-id-daa407ebe2a1eb7c61ffd59f75080c3b058cf39b/> quit before clear all callback to try fix sudoku can't quit
 - <csr-id-2cb79f715be8285dc5742d1d4a93ad4330c8f8e5/> move setup_logger function to log.rs
 - <csr-id-7fb308c2a7b9f9084263a627045260621c386733/> use /tmp/.dkmount* as tmp mount point
 - <csr-id-769f48882f6797debece8c5c7022a4b5c6aa326a/> fix gen_ssh_key log text (Retro -> Non-Retro)
 - <csr-id-723332a1309a1f23209998a07a396540822f1a9e/> cargo update
 - <csr-id-952dd934ea7fa353513f969ff0452b51515d3281/> update to clap4
   - Fix no_swap and swap_size conflicts

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 28 calendar days.
 - 28 days passed between releases.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.1 ([`fd284ec`](https://github.com/AOSC-Dev/aoscdk-rs/commit/fd284ec3ed3dbe9bcef357a2228081a8986d06ac))
    - Fix grub install to efi system in Linux 6.0 kernel ([`2ffbb53`](https://github.com/AOSC-Dev/aoscdk-rs/commit/2ffbb5306034004223fc453ce5234415b2b056be))
    - Fix cursive logger view ([`0f5f5bf`](https://github.com/AOSC-Dev/aoscdk-rs/commit/0f5f5bf67a735e7d7e25cc0be173f4fc75a0871b))
    - Quit before clear all callback to try fix sudoku can't quit ([`daa407e`](https://github.com/AOSC-Dev/aoscdk-rs/commit/daa407ebe2a1eb7c61ffd59f75080c3b058cf39b))
    - Move setup_logger function to log.rs ([`2cb79f7`](https://github.com/AOSC-Dev/aoscdk-rs/commit/2cb79f715be8285dc5742d1d4a93ad4330c8f8e5))
    - Use /tmp/.dkmount* as tmp mount point ([`7fb308c`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7fb308c2a7b9f9084263a627045260621c386733))
    - Fix gen_ssh_key log text (Retro -> Non-Retro) ([`769f488`](https://github.com/AOSC-Dev/aoscdk-rs/commit/769f48882f6797debece8c5c7022a4b5c6aa326a))
    - Cargo update ([`723332a`](https://github.com/AOSC-Dev/aoscdk-rs/commit/723332a1309a1f23209998a07a396540822f1a9e))
    - Update to clap4 ([`952dd93`](https://github.com/AOSC-Dev/aoscdk-rs/commit/952dd934ea7fa353513f969ff0452b51515d3281))
    - (cargo-release) start next development iteration 0.5.1-alpha.0 ([`bd20d84`](https://github.com/AOSC-Dev/aoscdk-rs/commit/bd20d84a579f7e8edc9f1fec0ec2bbf8e244af21))
</details>

## v0.5.0 (2022-09-21)

<csr-id-4dd70147ac02d45b047e6ed382b84f5fb77e5897/>

### Other

 - <csr-id-4dd70147ac02d45b047e6ed382b84f5fb77e5897/> bump to 5.0

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 ([`1761b65`](https://github.com/AOSC-Dev/aoscdk-rs/commit/1761b65dbee2a1426b382333f0c7a24f44f218aa))
    - Bump to 5.0 ([`4dd7014`](https://github.com/AOSC-Dev/aoscdk-rs/commit/4dd70147ac02d45b047e6ed382b84f5fb77e5897))
    - (cargo-release) start next development iteration 0.4.15-alpha.0 ([`a2f84d8`](https://github.com/AOSC-Dev/aoscdk-rs/commit/a2f84d858a4dc9a2eaec2fbc67526ec9b43bde27))
</details>

## v0.4.14 (2022-09-21)

<csr-id-5901afdecd6b4479ee2982f18eba040ab3d3275c/>
<csr-id-cd8484800be267da60a67e936a4201ff643c074e/>
<csr-id-c53396ba9358b7a356542b5ecf3ba1d3070c5edb/>
<csr-id-175a972da91b6def9d50965cffec71b12ae04547/>
<csr-id-e1eef72680b53b34fcf10cacca6d0267ac3fa945/>
<csr-id-8917eabb2a7f072fbef986b043a4878b12a1d4ec/>
<csr-id-a7100be405794d7e6c36ea44952c7fb9097f9912/>
<csr-id-678bddd415951f45c70f1bb9a00e011d9bc13dce/>
<csr-id-f1d5a33a72365fcb8d3ef4d9e0e630483136dfa3/>
<csr-id-42982b9c53a40099f14ea9a56149a16d0aa88c5e/>
<csr-id-57bb9db4adb382699f0b04378176b722b500268c/>
<csr-id-635a11e1de0f85e5b23039dd7ab792847eea0bc1/>
<csr-id-bb2c51698cbd3960597e34abab587e39336ff597/>

### Other

 - <csr-id-5901afdecd6b4479ee2982f18eba040ab3d3275c/> run cargo update
 - <csr-id-cd8484800be267da60a67e936a4201ff643c074e/> log console -> installer log
 - <csr-id-c53396ba9358b7a356542b5ecf3ba1d3070c5edb/> only write cpu0 info to log
 - <csr-id-175a972da91b6def9d50965cffec71b12ae04547/> log system info
 - <csr-id-e1eef72680b53b34fcf10cacca6d0267ac3fa945/> use sysinfo replace sys-info
 - <csr-id-8917eabb2a7f072fbef986b043a4878b12a1d4ec/> fill execute_grub_install function log
 - <csr-id-a7100be405794d7e6c36ea44952c7fb9097f9912/> fix copy log file
   - install: fix efi argument log write
 - <csr-id-678bddd415951f45c70f1bb9a00e011d9bc13dce/> fix a typo
 - <csr-id-f1d5a33a72365fcb8d3ef4d9e0e630483136dfa3/> improve error msg
   - set show_msgm, show_error width as 80
 - <csr-id-42982b9c53a40099f14ea9a56149a16d0aa88c5e/> run cargo clippy
 - <csr-id-57bb9db4adb382699f0b04378176b722b500268c/> installed after copy log file to main partition
   - Remove unnecessary test case
 - <csr-id-635a11e1de0f85e5b23039dd7ab792847eea0bc1/> add log
 - <csr-id-bb2c51698cbd3960597e34abab587e39336ff597/> adjust allow_fs_type view UI text

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 19 commits contributed to the release over the course of 19 calendar days.
 - 19 days passed between releases.
 - 13 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.14 ([`7822e8a`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7822e8a3e30cd907427da4b0e92e7d94790721f2))
    - Run cargo update ([`5901afd`](https://github.com/AOSC-Dev/aoscdk-rs/commit/5901afdecd6b4479ee2982f18eba040ab3d3275c))
    - Log console -> installer log ([`cd84848`](https://github.com/AOSC-Dev/aoscdk-rs/commit/cd8484800be267da60a67e936a4201ff643c074e))
    - Only write cpu0 info to log ([`c53396b`](https://github.com/AOSC-Dev/aoscdk-rs/commit/c53396ba9358b7a356542b5ecf3ba1d3070c5edb))
    - Tui, cli: abstruct run_command ([`539f09e`](https://github.com/AOSC-Dev/aoscdk-rs/commit/539f09ee5861be26201cb915835014de9407069d))
    - Log system info ([`175a972`](https://github.com/AOSC-Dev/aoscdk-rs/commit/175a972da91b6def9d50965cffec71b12ae04547))
    - Use sysinfo replace sys-info ([`e1eef72`](https://github.com/AOSC-Dev/aoscdk-rs/commit/e1eef72680b53b34fcf10cacca6d0267ac3fa945))
    - Fill execute_grub_install function log ([`8917eab`](https://github.com/AOSC-Dev/aoscdk-rs/commit/8917eabb2a7f072fbef986b043a4878b12a1d4ec))
    - Fix copy log file ([`a7100be`](https://github.com/AOSC-Dev/aoscdk-rs/commit/a7100be405794d7e6c36ea44952c7fb9097f9912))
    - Fix a typo ([`678bddd`](https://github.com/AOSC-Dev/aoscdk-rs/commit/678bddd415951f45c70f1bb9a00e011d9bc13dce))
    - Improve error msg ([`f1d5a33`](https://github.com/AOSC-Dev/aoscdk-rs/commit/f1d5a33a72365fcb8d3ef4d9e0e630483136dfa3))
    - Tui, cli: set log filename as datetime; fix log write ([`6e7cfba`](https://github.com/AOSC-Dev/aoscdk-rs/commit/6e7cfbada27f428f54c8568ca8c517964bc6c6cc))
    - Run cargo clippy ([`42982b9`](https://github.com/AOSC-Dev/aoscdk-rs/commit/42982b9c53a40099f14ea9a56149a16d0aa88c5e))
    - Installed after copy log file to main partition ([`57bb9db`](https://github.com/AOSC-Dev/aoscdk-rs/commit/57bb9db4adb382699f0b04378176b722b500268c))
    - Tui, cli: write log to /var/log ([`0e7275f`](https://github.com/AOSC-Dev/aoscdk-rs/commit/0e7275f80cfada27595efbbea61d69f72eed5742))
    - Install, mod, tui: improve log info ([`69e378e`](https://github.com/AOSC-Dev/aoscdk-rs/commit/69e378ecd277a6c239662b1c87910a915be18313))
    - Add log ([`635a11e`](https://github.com/AOSC-Dev/aoscdk-rs/commit/635a11e1de0f85e5b23039dd7ab792847eea0bc1))
    - Adjust allow_fs_type view UI text ([`bb2c516`](https://github.com/AOSC-Dev/aoscdk-rs/commit/bb2c51698cbd3960597e34abab587e39336ff597))
    - (cargo-release) start next development iteration 0.4.14-alpha.0 ([`705b8a0`](https://github.com/AOSC-Dev/aoscdk-rs/commit/705b8a082cf79db7f329cf809e6810d8c252e41d))
</details>

## v0.4.13 (2022-09-01)

<csr-id-5c9ceba1fe8876bfd9d86cc48214ef7e15758621/>
<csr-id-fefd35ffa35781a6181cf4c1edc1c3e35305e25a/>
<csr-id-8165c38e4d7ff8eb0c95db8315bc46f074724117/>
<csr-id-f3ff78a2d259412e7cb87cd584258652f4554a9d/>
<csr-id-429fad73c54a6d63432402967e6ad38363968a27/>
<csr-id-3b00de4a884f2b348db67e94c4be8c8f0504fd9c/>
<csr-id-b38230023575ab39c273cb0db4d9ef4a6daffedc/>
<csr-id-1ca13f6b06360f3184f7aea96ab2db0acfc2dd56/>
<csr-id-80cd2f8ddd4cab09059a7298fb1fcb0b5a75c39c/>
<csr-id-26540d551091f6812c906f355d84e7206cbd8d0a/>
<csr-id-8c3ff583be8d0d32094c153871dd8ffc16ec41bd/>
<csr-id-743dde5faac60d126088d762268d8629d5586876/>
<csr-id-2deac5aebe287b03fa76354b0d5c4ea591499638/>
<csr-id-87207cae79983345158ba6892880d5e9d5825b67/>
<csr-id-9d85ca985ea023dfbaacd949eb4772bf1a4626e5/>

### Other

 - <csr-id-5c9ceba1fe8876bfd9d86cc48214ef7e15758621/> drop new_partition_tabl function
   This function can not handle raw btrfs/md/luks ...This feature may corrupt the data of these users, so drop
 - <csr-id-fefd35ffa35781a6181cf4c1edc1c3e35305e25a/> do not return error in prepare_try_umount
 - <csr-id-8165c38e4d7ff8eb0c95db8315bc46f074724117/> improve logic in prepare umount
 - <csr-id-f3ff78a2d259412e7cb87cd584258652f4554a9d/> install before umount deploykit mount partition
 - <csr-id-429fad73c54a6d63432402967e6ad38363968a27/> update
 - <csr-id-3b00de4a884f2b348db67e94c4be8c8f0504fd9c/> use cargo clippy
 - <csr-id-b38230023575ab39c273cb0db4d9ef4a6daffedc/> update
 - <csr-id-1ca13f6b06360f3184f7aea96ab2db0acfc2dd56/> limit minesweeper open only have DISPLAY var
 - <csr-id-80cd2f8ddd4cab09059a7298fb1fcb0b5a75c39c/> fix start tips display
 - <csr-id-26540d551091f6812c906f355d84e7206cbd8d0a/> fix write fstab entry
 - <csr-id-8c3ff583be8d0d32094c153871dd8ffc16ec41bd/> fix swapon in install progress
 - <csr-id-743dde5faac60d126088d762268d8629d5586876/> improve error output
 - <csr-id-2deac5aebe287b03fa76354b0d5c4ea591499638/> cargo update
 - <csr-id-87207cae79983345158ba6892880d5e9d5825b67/> add confirmation to partitioning view
   - Use cargo clippy and cargo fmt to lint code
 - <csr-id-9d85ca985ea023dfbaacd949eb4772bf1a4626e5/> improve username invalid prompt

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 17 commits contributed to the release over the course of 26 calendar days.
 - 26 days passed between releases.
 - 15 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.13 ([`fc8f789`](https://github.com/AOSC-Dev/aoscdk-rs/commit/fc8f78972c38aed3d90e1f045c9cfeaaafba3b56))
    - Drop new_partition_tabl function ([`5c9ceba`](https://github.com/AOSC-Dev/aoscdk-rs/commit/5c9ceba1fe8876bfd9d86cc48214ef7e15758621))
    - Do not return error in prepare_try_umount ([`fefd35f`](https://github.com/AOSC-Dev/aoscdk-rs/commit/fefd35ffa35781a6181cf4c1edc1c3e35305e25a))
    - Improve logic in prepare umount ([`8165c38`](https://github.com/AOSC-Dev/aoscdk-rs/commit/8165c38e4d7ff8eb0c95db8315bc46f074724117))
    - Install before umount deploykit mount partition ([`f3ff78a`](https://github.com/AOSC-Dev/aoscdk-rs/commit/f3ff78a2d259412e7cb87cd584258652f4554a9d))
    - Update ([`429fad7`](https://github.com/AOSC-Dev/aoscdk-rs/commit/429fad73c54a6d63432402967e6ad38363968a27))
    - Use cargo clippy ([`3b00de4`](https://github.com/AOSC-Dev/aoscdk-rs/commit/3b00de4a884f2b348db67e94c4be8c8f0504fd9c))
    - Update ([`b382300`](https://github.com/AOSC-Dev/aoscdk-rs/commit/b38230023575ab39c273cb0db4d9ef4a6daffedc))
    - Limit minesweeper open only have DISPLAY var ([`1ca13f6`](https://github.com/AOSC-Dev/aoscdk-rs/commit/1ca13f6b06360f3184f7aea96ab2db0acfc2dd56))
    - Fix start tips display ([`80cd2f8`](https://github.com/AOSC-Dev/aoscdk-rs/commit/80cd2f8ddd4cab09059a7298fb1fcb0b5a75c39c))
    - Fix write fstab entry ([`26540d5`](https://github.com/AOSC-Dev/aoscdk-rs/commit/26540d551091f6812c906f355d84e7206cbd8d0a))
    - Fix swapon in install progress ([`8c3ff58`](https://github.com/AOSC-Dev/aoscdk-rs/commit/8c3ff583be8d0d32094c153871dd8ffc16ec41bd))
    - Improve error output ([`743dde5`](https://github.com/AOSC-Dev/aoscdk-rs/commit/743dde5faac60d126088d762268d8629d5586876))
    - Cargo update ([`2deac5a`](https://github.com/AOSC-Dev/aoscdk-rs/commit/2deac5aebe287b03fa76354b0d5c4ea591499638))
    - Add confirmation to partitioning view ([`87207ca`](https://github.com/AOSC-Dev/aoscdk-rs/commit/87207cae79983345158ba6892880d5e9d5825b67))
    - Improve username invalid prompt ([`9d85ca9`](https://github.com/AOSC-Dev/aoscdk-rs/commit/9d85ca985ea023dfbaacd949eb4772bf1a4626e5))
    - (cargo-release) start next development iteration 0.4.13-alpha.0 ([`a6b7327`](https://github.com/AOSC-Dev/aoscdk-rs/commit/a6b7327437dfe46619cf8ba072868120f89e0ed6))
</details>

## v0.4.12 (2022-08-05)

<csr-id-c7d827c0fcc34c198cad2cadcba6b50157584d7b/>
<csr-id-4a92ad27e95a9e402c2115dff0756d24aefefbbf/>
<csr-id-eadb58a92b7bae47bfb76456ba42170ac6cef364/>
<csr-id-8ef69ddb7e3d66858e0298818192d7b2342c6451/>

### Other

 - <csr-id-c7d827c0fcc34c198cad2cadcba6b50157584d7b/> cargo update
 - <csr-id-4a92ad27e95a9e402c2115dff0756d24aefefbbf/> fix stupid swap view logic
 - <csr-id-eadb58a92b7bae47bfb76456ba42170ac6cef364/> if can not get stdin, do not panic and return error
 - <csr-id-8ef69ddb7e3d66858e0298818192d7b2342c6451/> do not panic if /etc/adjtime line count < 2

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 9 calendar days.
 - 9 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.12 ([`9e3c963`](https://github.com/AOSC-Dev/aoscdk-rs/commit/9e3c963403fe3b1c9918982bf608e06567ba375e))
    - Cargo update ([`c7d827c`](https://github.com/AOSC-Dev/aoscdk-rs/commit/c7d827c0fcc34c198cad2cadcba6b50157584d7b))
    - Fix stupid swap view logic ([`4a92ad2`](https://github.com/AOSC-Dev/aoscdk-rs/commit/4a92ad27e95a9e402c2115dff0756d24aefefbbf))
    - If can not get stdin, do not panic and return error ([`eadb58a`](https://github.com/AOSC-Dev/aoscdk-rs/commit/eadb58a92b7bae47bfb76456ba42170ac6cef364))
    - Do not panic if /etc/adjtime line count < 2 ([`8ef69dd`](https://github.com/AOSC-Dev/aoscdk-rs/commit/8ef69ddb7e3d66858e0298818192d7b2342c6451))
    - (cargo-release) start next development iteration 0.4.12-alpha.0 ([`7c381d5`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7c381d5b572ae98c62a7f21484134c4053312b9a))
</details>

## v0.4.11 (2022-07-26)

<csr-id-1b52d86e4bc44ee12e84abc5f68f480dfa116e4e/>
<csr-id-0313600b6fc192725d7d518944c49608fc94efb4/>
<csr-id-14c0934d7d78d7fcb0d59d7001d85f08ab9cb7c6/>
<csr-id-e248e6525d0c1ffc35f17301745badb338e50b70/>
<csr-id-b8f31419cc99ae7d3496007c736c2afb7a202d08/>
<csr-id-b8f77685910064cc7415c5384f653a7217784001/>
<csr-id-3d1966edb95cafc2ee057ba47da9a2de2cd57a7d/>
<csr-id-980fc115e28584c9d4367ebc0a6606b7adf593c9/>

### Other

 - <csr-id-1b52d86e4bc44ee12e84abc5f68f480dfa116e4e/> adjust select_variant view width again
 - <csr-id-0313600b6fc192725d7d518944c49608fc94efb4/> adjust select_variant view width
 - <csr-id-14c0934d7d78d7fcb0d59d7001d85f08ab9cb7c6/> improve game-related strings
 - <csr-id-e248e6525d0c1ffc35f17301745badb338e50b70/> specify more conditions for swapfile creation
   Disallow swapfile if free space is less than 5GiB after creation
 - <csr-id-b8f31419cc99ae7d3496007c736c2afb7a202d08/> some changes in create_swapfile function
   - New line to swapfile fstab
   - swapon swapfile in AOSC OS/Retro
 - <csr-id-b8f77685910064cc7415c5384f653a7217784001/> remove useless execute_locale_gen function
   We are going to integrate pre-compiled locale on AOSC OS/Retro, so this function is no longer useful
 - <csr-id-3d1966edb95cafc2ee057ba47da9a2de2cd57a7d/> use cargo fmt
 - <csr-id-980fc115e28584c9d4367ebc0a6606b7adf593c9/> remove useless button; add undo key bind

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.11 ([`4babc2e`](https://github.com/AOSC-Dev/aoscdk-rs/commit/4babc2e6e785c2d7ce850f6e415405d5bd87a5c6))
    - Adjust select_variant view width again ([`1b52d86`](https://github.com/AOSC-Dev/aoscdk-rs/commit/1b52d86e4bc44ee12e84abc5f68f480dfa116e4e))
    - Adjust select_variant view width ([`0313600`](https://github.com/AOSC-Dev/aoscdk-rs/commit/0313600b6fc192725d7d518944c49608fc94efb4))
    - Improve game-related strings ([`14c0934`](https://github.com/AOSC-Dev/aoscdk-rs/commit/14c0934d7d78d7fcb0d59d7001d85f08ab9cb7c6))
    - Specify more conditions for swapfile creation ([`e248e65`](https://github.com/AOSC-Dev/aoscdk-rs/commit/e248e6525d0c1ffc35f17301745badb338e50b70))
    - Some changes in create_swapfile function ([`b8f3141`](https://github.com/AOSC-Dev/aoscdk-rs/commit/b8f31419cc99ae7d3496007c736c2afb7a202d08))
    - Remove useless execute_locale_gen function ([`b8f7768`](https://github.com/AOSC-Dev/aoscdk-rs/commit/b8f77685910064cc7415c5384f653a7217784001))
    - Use cargo fmt ([`3d1966e`](https://github.com/AOSC-Dev/aoscdk-rs/commit/3d1966edb95cafc2ee057ba47da9a2de2cd57a7d))
    - Remove useless button; add undo key bind ([`980fc11`](https://github.com/AOSC-Dev/aoscdk-rs/commit/980fc115e28584c9d4367ebc0a6606b7adf593c9))
    - (cargo-release) start next development iteration 0.4.11-alpha.0 ([`44ad69b`](https://github.com/AOSC-Dev/aoscdk-rs/commit/44ad69b279585b43068f5f2e97ad2926586da5e9))
</details>

## v0.4.10 (2022-07-26)

<csr-id-d8c1243cb6b5fcf05d70ccda286099bc9e257871/>
<csr-id-60ab90d8c9fb76aed11e1a2f0c620c5362cb9c28/>

### Other

 - <csr-id-d8c1243cb6b5fcf05d70ccda286099bc9e257871/> fix hostname uppercase
 - <csr-id-60ab90d8c9fb76aed11e1a2f0c620c5362cb9c28/> use cargo fmt

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.10 ([`54151f9`](https://github.com/AOSC-Dev/aoscdk-rs/commit/54151f9678a31bfada7698c861aef7a2b9274f18))
    - Fix hostname uppercase ([`d8c1243`](https://github.com/AOSC-Dev/aoscdk-rs/commit/d8c1243cb6b5fcf05d70ccda286099bc9e257871))
    - Use cargo fmt ([`60ab90d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/60ab90d8c9fb76aed11e1a2f0c620c5362cb9c28))
    - (cargo-release) start next development iteration 0.4.10-alpha.0 ([`202413f`](https://github.com/AOSC-Dev/aoscdk-rs/commit/202413f4ae19a430a853b779c000f7825896c2f5))
</details>

## v0.4.9 (2022-07-25)

<csr-id-799c76d28693a019af080f40e2691939a89660ad/>
<csr-id-87e3c78d04b83e014acf03fe380495f210ea4ef5/>
<csr-id-3a234b7c252a5d66ab421a0f34f5f7e834bc9f98/>
<csr-id-d5c25d938417837520b5d38bbb0d03bf9c4be2bb/>
<csr-id-d5491961c6792a95d204a88d5b6f8a83023c8b92/>
<csr-id-7495a568b08e53a06e0f67798fe4d05cf147913e/>
<csr-id-be40d45e1aee77eeab8a90201d57188a3a8f45c4/>
<csr-id-3c1721d29f0fe6e562da0221822146953c6bbcea/>
<csr-id-e780acb21efb9c3e91adcf2a59170fa3dc4e749a/>
<csr-id-deeae2ffc73f7676f05205a9f96f5a4934409670/>
<csr-id-4ac91c535eff698f7c4bb29eb234eb06a1fa030e/>
<csr-id-f7c55f74d0d349850952e21f9456df243a0dfee3/>
<csr-id-d9184867d5b2d95c87ac470ff41142fbdb9eff12/>
<csr-id-98f58bddcb674cacb463bbb829112131ff038fde/>
<csr-id-556493770e77c4fde5eb3663cc819b1422c41105/>
<csr-id-601365f0a267e485c762cf337f20130a542ef0b7/>
<csr-id-e63b2aedf6f55316179b61d6b9fd57f914f81ff8/>
<csr-id-bfb8a647b3202e034b8aee9e69fb7c7f046b70d2/>
<csr-id-05594fad6cb1f145cd5f16b369296accdc7a02e5/>
<csr-id-a2df96b9afb2e8646a2b9bc2af22afe7020e565f/>
<csr-id-39af0f57628ba8548b3a31f3b30096d410480e02/>
<csr-id-92a22bb90d1e33888a45a09a6fe4c5bdcfa938b0/>
<csr-id-c477e309235d2b56c4aaf587a725589bc51fa1c6/>
<csr-id-4ff90c1dd6cdaa039935e600180588dd0638ee4b/>
<csr-id-345446152075a48f63a2679bd8bff577da2fed17/>
<csr-id-ade21fddffaa31cec7f1156c70045044eacf07b3/>
<csr-id-769fc03bda15bfc209a985264fa3e716c3b6f151/>
<csr-id-d7f9a5a360b59fde10da349172b881f5b28eb3b2/>
<csr-id-1838ff931af19e1a577632a0483fbda48ef70d1b/>
<csr-id-ad1915b624c91b352727d895b5c9364ff76ad32b/>
<csr-id-31de9d17f622200efbed62216f90f095d7755d49/>
<csr-id-4e16a437561402d7487a4a072eecd9209ff609b1/>
<csr-id-dd11b459e7942b4cff94beffdb0cfe6ea8482ec6/>
<csr-id-9c4d2512b35d43cd379d277728505bee143d08c6/>
<csr-id-db829ba64057b3b307f471c8946228cdd426db1b/>
<csr-id-639c636f799c6a49798f0479cc3486fecc432165/>
<csr-id-9ff27ce519476f62bea20b1dd44726e6c04e79de/>
<csr-id-a685bab9728c1227a2209ba8d170e545dc235c72/>
<csr-id-811d072c552ececa1556b92e7df729813ee01eb5/>
<csr-id-128973b2bd7bcf99b37d8f33dc8875a47222ac8b/>
<csr-id-70dc73695af00883ef85e966f764e69910a5c6bc/>

### Other

 - <csr-id-799c76d28693a019af080f40e2691939a89660ad/> fix build on i486 arch
 - <csr-id-87e3c78d04b83e014acf03fe380495f210ea4ef5/> add a missing period point at hostname setup
 - <csr-id-3a234b7c252a5d66ab421a0f34f5f7e834bc9f98/> run cargo fmt
 - <csr-id-d5c25d938417837520b5d38bbb0d03bf9c4be2bb/> fix a UI text typo
 - <csr-id-d5491961c6792a95d204a88d5b6f8a83023c8b92/> lint UI strings
 - <csr-id-7495a568b08e53a06e0f67798fe4d05cf147913e/> device file => block device node
 - <csr-id-be40d45e1aee77eeab8a90201d57188a3a8f45c4/> fix output typo
 - <csr-id-3c1721d29f0fe6e562da0221822146953c6bbcea/> improve output if could not get user input tarball name
 - <csr-id-e780acb21efb9c3e91adcf2a59170fa3dc4e749a/> move get_variants func to network.rs
 - <csr-id-deeae2ffc73f7676f05205a9f96f5a4934409670/> add arg --list-tarball
 - <csr-id-4ac91c535eff698f7c4bb29eb234eb06a1fa030e/> set default username and password as aosc
 - <csr-id-f7c55f74d0d349850952e21f9456df243a0dfee3/> check hostname and username is accept
 - <csr-id-d9184867d5b2d95c87ac470ff41142fbdb9eff12/> run cargo fmt
 - <csr-id-98f58bddcb674cacb463bbb829112131ff038fde/> update dependices
 - <csr-id-556493770e77c4fde5eb3663cc819b1422c41105/> add search loclae view
 - <csr-id-601365f0a267e485c762cf337f20130a542ef0b7/> adjust timezone UI text
 - <csr-id-e63b2aedf6f55316179b61d6b9fd57f914f81ff8/> add UTC to zoneinfo list; set UTC as default zoneinfo
 - <csr-id-bfb8a647b3202e034b8aee9e69fb7c7f046b70d2/> improve is_acceptable_username function
 - <csr-id-05594fad6cb1f145cd5f16b369296accdc7a02e5/> do not allow user create name is 'root' account
 - <csr-id-a2df96b9afb2e8646a2b9bc2af22afe7020e565f/> only allow if DISPLAY exist play minesweeper
 - <csr-id-39af0f57628ba8548b3a31f3b30096d410480e02/> check username is lowercase
 - <csr-id-92a22bb90d1e33888a45a09a6fe4c5bdcfa938b0/> add hostname and username check
 - <csr-id-c477e309235d2b56c4aaf587a725589bc51fa1c6/> fix build again
   fallocate requires a different type under 32-bit than under 64-bit, with the former requiring an i32 and the latter requiring an i64
   
   This commit, if used on a 64-bit system, will convert the i32 to i64
 - <csr-id-4ff90c1dd6cdaa039935e600180588dd0638ee4b/> update dep
 - <csr-id-345446152075a48f63a2679bd8bff577da2fed17/> fix install retro with use C.UTF-8 locale
   - Also fix retro build (but use Rust 2021 Edition, wait AOSC OS/retro update rustc to 1.57 after ...
 - <csr-id-ade21fddffaa31cec7f1156c70045044eacf07b3/> add start game tips
 - <csr-id-769fc03bda15bfc209a985264fa3e716c3b6f151/> use cargo-fmt
 - <csr-id-d7f9a5a360b59fde10da349172b881f5b28eb3b2/> improve logic
   - Add clear_callback and add_callback to handle key callback
   - Add sudoku manual msg
 - <csr-id-1838ff931af19e1a577632a0483fbda48ef70d1b/> run cargo update
 - <csr-id-ad1915b624c91b352727d895b5c9364ff76ad32b/> run cargo-clippy and cargo-fmt
 - <csr-id-31de9d17f622200efbed62216f90f095d7755d49/> new game!
 - <csr-id-4e16a437561402d7487a4a072eecd9209ff609b1/> fix directory name typo
 - <csr-id-dd11b459e7942b4cff94beffdb0cfe6ea8482ec6/> run cargo-fmt again
 - <csr-id-9c4d2512b35d43cd379d277728505bee143d08c6/> run cargo-clippy again
 - <csr-id-db829ba64057b3b307f471c8946228cdd426db1b/> run cargo fmt and cargo clippy
 - <csr-id-639c636f799c6a49798f0479cc3486fecc432165/> how about a game of Minesweeper during installation?
   Press 'm' and enjoy it~
 - <csr-id-9ff27ce519476f62bea20b1dd44726e6c04e79de/> select timezone new ui/ux
   - Thanks https://github.com/gyscos/cursive/blob/main/cursive/examples/autocomplete.rs !
 - <csr-id-a685bab9728c1227a2209ba8d170e545dc235c72/> add user to plugdev group
 - <csr-id-811d072c552ececa1556b92e7df729813ee01eb5/> update cursive to 0.18.0
   - Also cargo fmt
 - <csr-id-128973b2bd7bcf99b37d8f33dc8875a47222ac8b/> fix not enough space message
 - <csr-id-70dc73695af00883ef85e966f764e69910a5c6bc/> fix new_partition_table in ppc64/el

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 44 commits contributed to the release over the course of 76 calendar days.
 - 76 days passed between releases.
 - 41 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.9 ([`7cba3b3`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7cba3b3dd48d11fa0caed5cadce253969d0088fd))
    - Fix build on i486 arch ([`799c76d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/799c76d28693a019af080f40e2691939a89660ad))
    - Add a missing period point at hostname setup ([`87e3c78`](https://github.com/AOSC-Dev/aoscdk-rs/commit/87e3c78d04b83e014acf03fe380495f210ea4ef5))
    - Run cargo fmt ([`3a234b7`](https://github.com/AOSC-Dev/aoscdk-rs/commit/3a234b7c252a5d66ab421a0f34f5f7e834bc9f98))
    - Fix a UI text typo ([`d5c25d9`](https://github.com/AOSC-Dev/aoscdk-rs/commit/d5c25d938417837520b5d38bbb0d03bf9c4be2bb))
    - Lint UI strings ([`d549196`](https://github.com/AOSC-Dev/aoscdk-rs/commit/d5491961c6792a95d204a88d5b6f8a83023c8b92))
    - Device file => block device node ([`7495a56`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7495a568b08e53a06e0f67798fe4d05cf147913e))
    - Fix output typo ([`be40d45`](https://github.com/AOSC-Dev/aoscdk-rs/commit/be40d45e1aee77eeab8a90201d57188a3a8f45c4))
    - Improve output if could not get user input tarball name ([`3c1721d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/3c1721d29f0fe6e562da0221822146953c6bbcea))
    - Move get_variants func to network.rs ([`e780acb`](https://github.com/AOSC-Dev/aoscdk-rs/commit/e780acb21efb9c3e91adcf2a59170fa3dc4e749a))
    - Add arg --list-tarball ([`deeae2f`](https://github.com/AOSC-Dev/aoscdk-rs/commit/deeae2ffc73f7676f05205a9f96f5a4934409670))
    - Revert "frontend/cli: set default username and password as aosc" ([`2d35ed0`](https://github.com/AOSC-Dev/aoscdk-rs/commit/2d35ed0c6fc55afbffc8c17c75e515962c1fee8c))
    - Set default username and password as aosc ([`4ac91c5`](https://github.com/AOSC-Dev/aoscdk-rs/commit/4ac91c535eff698f7c4bb29eb234eb06a1fa030e))
    - Check hostname and username is accept ([`f7c55f7`](https://github.com/AOSC-Dev/aoscdk-rs/commit/f7c55f74d0d349850952e21f9456df243a0dfee3))
    - Run cargo fmt ([`d918486`](https://github.com/AOSC-Dev/aoscdk-rs/commit/d9184867d5b2d95c87ac470ff41142fbdb9eff12))
    - Update dependices ([`98f58bd`](https://github.com/AOSC-Dev/aoscdk-rs/commit/98f58bddcb674cacb463bbb829112131ff038fde))
    - Add search loclae view ([`5564937`](https://github.com/AOSC-Dev/aoscdk-rs/commit/556493770e77c4fde5eb3663cc819b1422c41105))
    - Adjust timezone UI text ([`601365f`](https://github.com/AOSC-Dev/aoscdk-rs/commit/601365f0a267e485c762cf337f20130a542ef0b7))
    - Add UTC to zoneinfo list; set UTC as default zoneinfo ([`e63b2ae`](https://github.com/AOSC-Dev/aoscdk-rs/commit/e63b2aedf6f55316179b61d6b9fd57f914f81ff8))
    - Improve is_acceptable_username function ([`bfb8a64`](https://github.com/AOSC-Dev/aoscdk-rs/commit/bfb8a647b3202e034b8aee9e69fb7c7f046b70d2))
    - Do not allow user create name is 'root' account ([`05594fa`](https://github.com/AOSC-Dev/aoscdk-rs/commit/05594fad6cb1f145cd5f16b369296accdc7a02e5))
    - Only allow if DISPLAY exist play minesweeper ([`a2df96b`](https://github.com/AOSC-Dev/aoscdk-rs/commit/a2df96b9afb2e8646a2b9bc2af22afe7020e565f))
    - Check username is lowercase ([`39af0f5`](https://github.com/AOSC-Dev/aoscdk-rs/commit/39af0f57628ba8548b3a31f3b30096d410480e02))
    - Add hostname and username check ([`92a22bb`](https://github.com/AOSC-Dev/aoscdk-rs/commit/92a22bb90d1e33888a45a09a6fe4c5bdcfa938b0))
    - Fix build again ([`c477e30`](https://github.com/AOSC-Dev/aoscdk-rs/commit/c477e309235d2b56c4aaf587a725589bc51fa1c6))
    - Update dep ([`4ff90c1`](https://github.com/AOSC-Dev/aoscdk-rs/commit/4ff90c1dd6cdaa039935e600180588dd0638ee4b))
    - Fix install retro with use C.UTF-8 locale ([`3454461`](https://github.com/AOSC-Dev/aoscdk-rs/commit/345446152075a48f63a2679bd8bff577da2fed17))
    - Add start game tips ([`ade21fd`](https://github.com/AOSC-Dev/aoscdk-rs/commit/ade21fddffaa31cec7f1156c70045044eacf07b3))
    - Use cargo-fmt ([`769fc03`](https://github.com/AOSC-Dev/aoscdk-rs/commit/769fc03bda15bfc209a985264fa3e716c3b6f151))
    - Improve logic ([`d7f9a5a`](https://github.com/AOSC-Dev/aoscdk-rs/commit/d7f9a5a360b59fde10da349172b881f5b28eb3b2))
    - Run cargo update ([`1838ff9`](https://github.com/AOSC-Dev/aoscdk-rs/commit/1838ff931af19e1a577632a0483fbda48ef70d1b))
    - Run cargo-clippy and cargo-fmt ([`ad1915b`](https://github.com/AOSC-Dev/aoscdk-rs/commit/ad1915b624c91b352727d895b5c9364ff76ad32b))
    - New game! ([`31de9d1`](https://github.com/AOSC-Dev/aoscdk-rs/commit/31de9d17f622200efbed62216f90f095d7755d49))
    - Fix directory name typo ([`4e16a43`](https://github.com/AOSC-Dev/aoscdk-rs/commit/4e16a437561402d7487a4a072eecd9209ff609b1))
    - Run cargo-fmt again ([`dd11b45`](https://github.com/AOSC-Dev/aoscdk-rs/commit/dd11b459e7942b4cff94beffdb0cfe6ea8482ec6))
    - Run cargo-clippy again ([`9c4d251`](https://github.com/AOSC-Dev/aoscdk-rs/commit/9c4d2512b35d43cd379d277728505bee143d08c6))
    - Run cargo fmt and cargo clippy ([`db829ba`](https://github.com/AOSC-Dev/aoscdk-rs/commit/db829ba64057b3b307f471c8946228cdd426db1b))
    - How about a game of Minesweeper during installation? ([`639c636`](https://github.com/AOSC-Dev/aoscdk-rs/commit/639c636f799c6a49798f0479cc3486fecc432165))
    - Select timezone new ui/ux ([`9ff27ce`](https://github.com/AOSC-Dev/aoscdk-rs/commit/9ff27ce519476f62bea20b1dd44726e6c04e79de))
    - Add user to plugdev group ([`a685bab`](https://github.com/AOSC-Dev/aoscdk-rs/commit/a685bab9728c1227a2209ba8d170e545dc235c72))
    - Update cursive to 0.18.0 ([`811d072`](https://github.com/AOSC-Dev/aoscdk-rs/commit/811d072c552ececa1556b92e7df729813ee01eb5))
    - Fix not enough space message ([`128973b`](https://github.com/AOSC-Dev/aoscdk-rs/commit/128973b2bd7bcf99b37d8f33dc8875a47222ac8b))
    - Fix new_partition_table in ppc64/el ([`70dc736`](https://github.com/AOSC-Dev/aoscdk-rs/commit/70dc73695af00883ef85e966f764e69910a5c6bc))
    - (cargo-release) start next development iteration 0.4.9-alpha.0 ([`4cb97c1`](https://github.com/AOSC-Dev/aoscdk-rs/commit/4cb97c1bff829fc7f98b3409dd1116615ca243b7))
</details>

## v0.4.8 (2022-05-09)

<csr-id-22d97b0437e83e752df1f0429e1e402054ad102f/>
<csr-id-39f67d142b3e81deae5d6d5ae7f598681e72a0ff/>
<csr-id-789d30334ab41dc7e1c551b457d494eb168cefd1/>
<csr-id-e5e92a410ccb62782d14dd40b81a5468e4b54e63/>
<csr-id-facbb4a04754e53918332ef39c904ffc08b3a46e/>
<csr-id-f881d928b27eb4d0b315e6930fd2b4722dc9a880/>
<csr-id-b0f542c5eed56572aa6d54254cc2e33c7c34e399/>

### Other

 - <csr-id-22d97b0437e83e752df1f0429e1e402054ad102f/> fix disable_hibernate
 - <csr-id-39f67d142b3e81deae5d6d5ae7f598681e72a0ff/> fix new_partition_table() in ppc64el
 - <csr-id-789d30334ab41dc7e1c551b457d494eb168cefd1/> fix get_partition_table_type on ppc64/el
 - <csr-id-e5e92a410ccb62782d14dd40b81a5468e4b54e63/> improve get_partition_table_type logic
 - <csr-id-facbb4a04754e53918332ef39c904ffc08b3a46e/> use cargo clippy
 - <csr-id-f881d928b27eb4d0b315e6930fd2b4722dc9a880/> improve check partition type
 - <csr-id-b0f542c5eed56572aa6d54254cc2e33c7c34e399/> select_partiton before create empty right partition table

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.8 ([`c46a7db`](https://github.com/AOSC-Dev/aoscdk-rs/commit/c46a7db0df20f1ce493cc4ac6a706d74b809c99b))
    - Fix disable_hibernate ([`22d97b0`](https://github.com/AOSC-Dev/aoscdk-rs/commit/22d97b0437e83e752df1f0429e1e402054ad102f))
    - Fix new_partition_table() in ppc64el ([`39f67d1`](https://github.com/AOSC-Dev/aoscdk-rs/commit/39f67d142b3e81deae5d6d5ae7f598681e72a0ff))
    - Fix get_partition_table_type on ppc64/el ([`789d303`](https://github.com/AOSC-Dev/aoscdk-rs/commit/789d30334ab41dc7e1c551b457d494eb168cefd1))
    - Improve get_partition_table_type logic ([`e5e92a4`](https://github.com/AOSC-Dev/aoscdk-rs/commit/e5e92a410ccb62782d14dd40b81a5468e4b54e63))
    - Use cargo clippy ([`facbb4a`](https://github.com/AOSC-Dev/aoscdk-rs/commit/facbb4a04754e53918332ef39c904ffc08b3a46e))
    - Improve check partition type ([`f881d92`](https://github.com/AOSC-Dev/aoscdk-rs/commit/f881d928b27eb4d0b315e6930fd2b4722dc9a880))
    - Select_partiton before create empty right partition table ([`b0f542c`](https://github.com/AOSC-Dev/aoscdk-rs/commit/b0f542c5eed56572aa6d54254cc2e33c7c34e399))
    - (cargo-release) start next development iteration 0.4.8-alpha.0 ([`80f896e`](https://github.com/AOSC-Dev/aoscdk-rs/commit/80f896e2f06709b389416714e500013db750372d))
</details>

## v0.4.7 (2022-05-08)

<csr-id-8a82a59be0d66957a6142fdf529dfa124e4298b6/>

### Other

 - <csr-id-8a82a59be0d66957a6142fdf529dfa124e4298b6/> default disable hibernate

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.7 ([`9a8d48e`](https://github.com/AOSC-Dev/aoscdk-rs/commit/9a8d48e7c2f5f7fd94dcd15dfb4751641ece2246))
    - Default disable hibernate ([`8a82a59`](https://github.com/AOSC-Dev/aoscdk-rs/commit/8a82a59be0d66957a6142fdf529dfa124e4298b6))
    - (cargo-release) start next development iteration 0.4.7-alpha.0 ([`dc6f616`](https://github.com/AOSC-Dev/aoscdk-rs/commit/dc6f6162438f3c4ef325bc566d65e054af9a5799))
</details>

## v0.4.6 (2022-05-07)

<csr-id-d158f4602e08bc00a8c870107d3d6cbefff0b2c4/>

### Other

 - <csr-id-d158f4602e08bc00a8c870107d3d6cbefff0b2c4/> improve disable_hibernate logic

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.6 ([`d575272`](https://github.com/AOSC-Dev/aoscdk-rs/commit/d5752722c632017b2b4b320411c816b7cb9dbcf2))
    - Improve disable_hibernate logic ([`d158f46`](https://github.com/AOSC-Dev/aoscdk-rs/commit/d158f4602e08bc00a8c870107d3d6cbefff0b2c4))
    - (cargo-release) start next development iteration 0.4.6-alpha.0 ([`56cfb73`](https://github.com/AOSC-Dev/aoscdk-rs/commit/56cfb7321a53ffadc64ea53ce4b3907314c2541e))
</details>

## v0.4.5 (2022-05-07)

<csr-id-7d39a0650c6689531e10455dbe99293dfb777ccb/>

### Other

 - <csr-id-7d39a0650c6689531e10455dbe99293dfb777ccb/> add disable hibernate function

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.5 ([`b020c48`](https://github.com/AOSC-Dev/aoscdk-rs/commit/b020c48da062a36e30b6499d842e8198b5e24e9f))
    - Add disable hibernate function ([`7d39a06`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7d39a0650c6689531e10455dbe99293dfb777ccb))
    - (cargo-release) start next development iteration 0.4.5-alpha.0 ([`cddde58`](https://github.com/AOSC-Dev/aoscdk-rs/commit/cddde58dedb9ab5e74d5365303596272d7aeac3e))
</details>

## v0.4.4 (2022-05-07)

<csr-id-3c5af6ae926151e150b6537785d2a8e09f7d9325/>
<csr-id-7e6723bbd784d44690ce7338444b08bec553eb3c/>
<csr-id-4ad132a27c3cb3c2e57d178887fc1ed677d663f0/>
<csr-id-01f59268dbd515ab0079b4aeb83f75d8cbdaf31f/>
<csr-id-260f9a017c6ebca83762a3799aded6aea7e4a4a3/>
<csr-id-9f31e7c14aee3e884f1e1f392400d7a429d6faa5/>
<csr-id-434127854099c2e436dff82d37de6a58a3ac4819/>
<csr-id-47fb62da885e2985ba71c5c2e638f9d2a47f7117/>
<csr-id-037e22ef62526810652a4871071924bb932f7498/>
<csr-id-eb95593d7d54bd0fd97dcf3222cf2a2e246b64c5/>
<csr-id-c8f59ea749bc263d5fe63a2d1d88c652629c6c9c/>
<csr-id-97df22e924378b063fddaa846e0656390fcd8a23/>

### Other

 - <csr-id-3c5af6ae926151e150b6537785d2a8e09f7d9325/> add swap summary
 - <csr-id-7e6723bbd784d44690ce7338444b08bec553eb3c/> Installer => AOSC OS Installer in welcome text
 - <csr-id-4ad132a27c3cb3c2e57d178887fc1ed677d663f0/> improve UI strings
   - Change all "AOSC OS Installer" subject => "Installer".
   - Use simple past tense.
   - Fix a typo in src/disks.rs.
 - <csr-id-01f59268dbd515ab0079b4aeb83f75d8cbdaf31f/> improve error message when aoscdk fails to obtain the instance lock
 - <csr-id-260f9a017c6ebca83762a3799aded6aea7e4a4a3/> Ok => OK
 - <csr-id-9f31e7c14aee3e884f1e1f392400d7a429d6faa5/> fix check bios+mbr or uefi+gpt
 - <csr-id-434127854099c2e436dff82d37de6a58a3ac4819/> improve partition listing
   Display unknown FS or unformatted partitions as (Unknown/Unformatted),
   instead of displaying a "?".
 - <csr-id-47fb62da885e2985ba71c5c2e638f9d2a47f7117/> only check ppc64el is gpt
 - <csr-id-037e22ef62526810652a4871071924bb932f7498/> check is gpt in ppc64/ppc64el
 - <csr-id-eb95593d7d54bd0fd97dcf3222cf2a2e246b64c5/> check uefi+gpt or bios+mbr in debug mode
 - <csr-id-c8f59ea749bc263d5fe63a2d1d88c652629c6c9c/> adapt check uefi+gpt or bios+mbr
 - <csr-id-97df22e924378b063fddaa846e0656390fcd8a23/> check UEFI+GPT or BIOS+MBR

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 3 calendar days.
 - 3 days passed between releases.
 - 12 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.4 ([`ab243db`](https://github.com/AOSC-Dev/aoscdk-rs/commit/ab243dbe72427b189bc95c56b56edc163459ca92))
    - Improve swap summary ([`c155afc`](https://github.com/AOSC-Dev/aoscdk-rs/commit/c155afceab5d7cf6c6a5750a51089585cd2fca78))
    - Add swap summary ([`3c5af6a`](https://github.com/AOSC-Dev/aoscdk-rs/commit/3c5af6ae926151e150b6537785d2a8e09f7d9325))
    - Installer => AOSC OS Installer in welcome text ([`7e6723b`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7e6723bbd784d44690ce7338444b08bec553eb3c))
    - Improve UI strings ([`4ad132a`](https://github.com/AOSC-Dev/aoscdk-rs/commit/4ad132a27c3cb3c2e57d178887fc1ed677d663f0))
    - Improve error message when aoscdk fails to obtain the instance lock ([`01f5926`](https://github.com/AOSC-Dev/aoscdk-rs/commit/01f59268dbd515ab0079b4aeb83f75d8cbdaf31f))
    - Ok => OK ([`260f9a0`](https://github.com/AOSC-Dev/aoscdk-rs/commit/260f9a017c6ebca83762a3799aded6aea7e4a4a3))
    - Fix check bios+mbr or uefi+gpt ([`9f31e7c`](https://github.com/AOSC-Dev/aoscdk-rs/commit/9f31e7c14aee3e884f1e1f392400d7a429d6faa5))
    - Improve partition listing ([`4341278`](https://github.com/AOSC-Dev/aoscdk-rs/commit/434127854099c2e436dff82d37de6a58a3ac4819))
    - Only check ppc64el is gpt ([`47fb62d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/47fb62da885e2985ba71c5c2e638f9d2a47f7117))
    - Check is gpt in ppc64/ppc64el ([`037e22e`](https://github.com/AOSC-Dev/aoscdk-rs/commit/037e22ef62526810652a4871071924bb932f7498))
    - Check uefi+gpt or bios+mbr in debug mode ([`eb95593`](https://github.com/AOSC-Dev/aoscdk-rs/commit/eb95593d7d54bd0fd97dcf3222cf2a2e246b64c5))
    - Adapt check uefi+gpt or bios+mbr ([`c8f59ea`](https://github.com/AOSC-Dev/aoscdk-rs/commit/c8f59ea749bc263d5fe63a2d1d88c652629c6c9c))
    - Check UEFI+GPT or BIOS+MBR ([`97df22e`](https://github.com/AOSC-Dev/aoscdk-rs/commit/97df22e924378b063fddaa846e0656390fcd8a23))
    - (cargo-release) start next development iteration 0.4.4-alpha.0 ([`c58b26d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/c58b26da334fb24b5000d14a78a915de3fef1814))
</details>

## v0.4.3 (2022-05-04)

<csr-id-a4aa4309471627d776d3bb45e13393be4a9a324e/>
<csr-id-4b238b6a957602c859a57e4d4b2bcb765ec4fb0d/>
<csr-id-1a7138d4295962ea4dbfd113e304cf3c29ae79b8/>
<csr-id-5a86932eb2cb86cf5b44e968ca4272be5174ac2c/>

### Other

 - <csr-id-a4aa4309471627d776d3bb45e13393be4a9a324e/> handle check cancel thread error
 - <csr-id-4b238b6a957602c859a57e4d4b2bcb765ec4fb0d/> use cargo fmt
 - <csr-id-1a7138d4295962ea4dbfd113e304cf3c29ae79b8/> move use std::env::consts::ARCH to get_arch_name() function to fix ppc64el build warming
 - <csr-id-5a86932eb2cb86cf5b44e968ca4272be5174ac2c/> fix nix::libc import

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 2 calendar days.
 - 2 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.3 ([`7cf5bfa`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7cf5bfa05dad6e594045c9ffc355847deede904e))
    - Handle check cancel thread error ([`a4aa430`](https://github.com/AOSC-Dev/aoscdk-rs/commit/a4aa4309471627d776d3bb45e13393be4a9a324e))
    - Use cargo fmt ([`4b238b6`](https://github.com/AOSC-Dev/aoscdk-rs/commit/4b238b6a957602c859a57e4d4b2bcb765ec4fb0d))
    - Move use std::env::consts::ARCH to get_arch_name() function to fix ppc64el build warming ([`1a7138d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/1a7138d4295962ea4dbfd113e304cf3c29ae79b8))
    - Fix nix::libc import ([`5a86932`](https://github.com/AOSC-Dev/aoscdk-rs/commit/5a86932eb2cb86cf5b44e968ca4272be5174ac2c))
    - (cargo-release) start next development iteration 0.4.3-alpha.0 ([`2835a7e`](https://github.com/AOSC-Dev/aoscdk-rs/commit/2835a7e6765f1e661451ef35752e6fa7a71b14df))
</details>

## v0.4.2 (2022-05-01)

<csr-id-7e35f87ec637ccc9af59a73f9a1ff60abdde3792/>
<csr-id-f0662762868d85d1922ca9adb3673f0f2160cbb2/>
<csr-id-892e193e84e0279105883613085b12049d158c2c/>
<csr-id-2bbd7e286f4a5224657dedf254bd424263968764/>
<csr-id-5a46583c26e860faed26a6e19db2023c85e875f2/>
<csr-id-4e7143232b9f1bdae87d00342a090ea60dc01372/>
<csr-id-76f206fc7b59008bf75faa69ac3af66de5828c04/>

### Other

 - <csr-id-7e35f87ec637ccc9af59a73f9a1ff60abdde3792/> fix fstab_entries() match esp partition
 - <csr-id-f0662762868d85d1922ca9adb3673f0f2160cbb2/> fix format esp partition
 - <csr-id-892e193e84e0279105883613085b12049d158c2c/> check if is EFI to adjust grub argument
 - <csr-id-2bbd7e286f4a5224657dedf254bd424263968764/> fix ppc grub install argument again
 - <csr-id-5a46583c26e860faed26a6e19db2023c85e875f2/> fix ppc grub install argument
 - <csr-id-4e7143232b9f1bdae87d00342a090ea60dc01372/> use cargo fmt
 - <csr-id-76f206fc7b59008bf75faa69ac3af66de5828c04/> try to fix other arch grub install
   - Also improve network::get_arch_name() from ciel-rs

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.2 ([`16dca0d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/16dca0d8b760c66649e2806edf6fb3527d823c21))
    - Fix fstab_entries() match esp partition ([`7e35f87`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7e35f87ec637ccc9af59a73f9a1ff60abdde3792))
    - Fix format esp partition ([`f066276`](https://github.com/AOSC-Dev/aoscdk-rs/commit/f0662762868d85d1922ca9adb3673f0f2160cbb2))
    - Check if is EFI to adjust grub argument ([`892e193`](https://github.com/AOSC-Dev/aoscdk-rs/commit/892e193e84e0279105883613085b12049d158c2c))
    - Fix ppc grub install argument again ([`2bbd7e2`](https://github.com/AOSC-Dev/aoscdk-rs/commit/2bbd7e286f4a5224657dedf254bd424263968764))
    - Fix ppc grub install argument ([`5a46583`](https://github.com/AOSC-Dev/aoscdk-rs/commit/5a46583c26e860faed26a6e19db2023c85e875f2))
    - Use cargo fmt ([`4e71432`](https://github.com/AOSC-Dev/aoscdk-rs/commit/4e7143232b9f1bdae87d00342a090ea60dc01372))
    - Try to fix other arch grub install ([`76f206f`](https://github.com/AOSC-Dev/aoscdk-rs/commit/76f206fc7b59008bf75faa69ac3af66de5828c04))
    - (cargo-release) start next development iteration 0.4.2-alpha.0 ([`5024abf`](https://github.com/AOSC-Dev/aoscdk-rs/commit/5024abfbafe99a9288382a2de1bdb36ad2734d6d))
</details>

## v0.4.1 (2022-05-01)

<csr-id-0e66a9dd412fdde768c7592c33945c8c6c37d733/>
<csr-id-8dbf6e5318f1179e19f5c2c59b5e4a487ac44d74/>

### Other

 - <csr-id-0e66a9dd412fdde768c7592c33945c8c6c37d733/> fix gen partition uuid
 - <csr-id-8dbf6e5318f1179e19f5c2c59b5e4a487ac44d74/> set default timezone as Asia/Shanghai to fix install

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.1 ([`e877966`](https://github.com/AOSC-Dev/aoscdk-rs/commit/e877966262860adf750a31064dd2150f18243a2b))
    - Fix gen partition uuid ([`0e66a9d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/0e66a9dd412fdde768c7592c33945c8c6c37d733))
    - Set default timezone as Asia/Shanghai to fix install ([`8dbf6e5`](https://github.com/AOSC-Dev/aoscdk-rs/commit/8dbf6e5318f1179e19f5c2c59b5e4a487ac44d74))
    - (cargo-release) start next development iteration 0.4.1-alpha.0 ([`5f8612e`](https://github.com/AOSC-Dev/aoscdk-rs/commit/5f8612ea073dde55b7e8243301b1ca0589bcb4b0))
</details>

## v0.4.0 (2022-04-30)

<csr-id-14a40076a484c8c4dbe0dd1f9f53169a6d33efba/>
<csr-id-72fe58a795803be96c78cb68d387f5cbc478e472/>
<csr-id-b6da46e09046a833309acb5001a7115729c3033e/>
<csr-id-ae8c9c5f24adfc9c979d00bf1d51f3e408bbaf4c/>
<csr-id-313b5efc93033fbab1272290d0ae1d71dfe82cf3/>
<csr-id-64d3bd6b8358f590eded85438a1e62f299af3476/>
<csr-id-1c7a0f1ef7706f27228cd847be06e0d0479625c4/>
<csr-id-50625d76669361c8e071344ec888b5102ae1d0d8/>
<csr-id-23753ead7aba413bdc5407a3e27443f3afd130d4/>
<csr-id-2715d44f198802fc26ed5b1a30b5929818688f1f/>
<csr-id-861f1abe3fe2421bd6bcb2ccea127be29664db6d/>
<csr-id-22fe212d141c3014f2b562db03a1006a40b6b940/>
<csr-id-8a18b889073e98d2b4a62375a1a87cff55c8163a/>
<csr-id-29db63bf15bc1680600068a2a09ef393c8830af4/>
<csr-id-fc11578b8c32ec0a4ddac77b2506145b20c56f17/>
<csr-id-8b613f22486fac4e08ca523b11aac08bf7048f1f/>
<csr-id-a99e553f2cddd13d6294a824d4b8fdf1e0874dd0/>
<csr-id-978c56867221ffb678eddad409e102390802241c/>
<csr-id-5f570363fb246968c2dc1a585a29900bf1a34c58/>
<csr-id-6b4c509c30909c6809fd7fa0d34955bedda89791/>
<csr-id-bf0e2dbdc9e37af9b32f6fc7ad72be701d94e8fa/>
<csr-id-c650b2d42492a159539b7901c7e9bca0620754fc/>
<csr-id-d6aa855faa6e7e342dad8d4cc66e296ffe22bd05/>
<csr-id-122f69057418328d59da9c7aeb184b027c117eef/>
<csr-id-eb73dc20b116de867cda93976ac4e5d8ae24b615/>
<csr-id-0ae6dcc7edeef0139c66eb0fdbc71c9f38cbd290/>

### Other

 - <csr-id-14a40076a484c8c4dbe0dd1f9f53169a6d33efba/> bump version to 0.4
 - <csr-id-72fe58a795803be96c78cb68d387f5cbc478e472/> fix some string
 - <csr-id-b6da46e09046a833309acb5001a7115729c3033e/> revise UI strings
 - <csr-id-ae8c9c5f24adfc9c979d00bf1d51f3e408bbaf4c/> use cargo clippy
 - <csr-id-313b5efc93033fbab1272290d0ae1d71dfe82cf3/> fix get lock
 - <csr-id-64d3bd6b8358f590eded85438a1e62f299af3476/> add process lock
   - Also improve match tarball variant name
 - <csr-id-1c7a0f1ef7706f27228cd847be06e0d0479625c4/> you are now a dungeon master! (thx @cth451)
 - <csr-id-50625d76669361c8e071344ec888b5102ae1d0d8/> fix print install finished
 - <csr-id-23753ead7aba413bdc5407a3e27443f3afd130d4/> fix join thread failed
 - <csr-id-2715d44f198802fc26ed5b1a30b5929818688f1f/> cancel install after exit
 - <csr-id-861f1abe3fe2421bd6bcb2ccea127be29664db6d/> fix error handle
 - <csr-id-22fe212d141c3014f2b562db03a1006a40b6b940/> use cargo clippy
 - <csr-id-8a18b889073e98d2b4a62375a1a87cff55c8163a/> fix auto swap
 - <csr-id-29db63bf15bc1680600068a2a09ef393c8830af4/> fix default mirror download
 - <csr-id-fc11578b8c32ec0a4ddac77b2506145b20c56f17/> add some feature
 - <csr-id-8b613f22486fac4e08ca523b11aac08bf7048f1f/> new
 - <csr-id-a99e553f2cddd13d6294a824d4b8fdf1e0874dd0/> fix cancel failed bug
 - <csr-id-978c56867221ffb678eddad409e102390802241c/> start_install view cancel install after umount fs
 - <csr-id-5f570363fb246968c2dc1a585a29900bf1a34c58/> cargo clippy
 - <csr-id-6b4c509c30909c6809fd7fa0d34955bedda89791/> start_install view add cancel button
 - <csr-id-bf0e2dbdc9e37af9b32f6fc7ad72be701d94e8fa/> improve select_swap UI logic
 - <csr-id-c650b2d42492a159539b7901c7e9bca0620754fc/> select_swap view add back and exit button
   - Also cargo clippy
 - <csr-id-d6aa855faa6e7e342dad8d4cc66e296ffe22bd05/> improve ui
 - <csr-id-122f69057418328d59da9c7aeb184b027c117eef/> fix set swapfile permission
 - <csr-id-eb73dc20b116de867cda93976ac4e5d8ae24b615/> add select_swap view to create swapfile
 - <csr-id-0ae6dcc7edeef0139c66eb0fdbc71c9f38cbd290/> add riscv64 architecture mapping

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 28 commits contributed to the release over the course of 70 calendar days.
 - 70 days passed between releases.
 - 26 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 ([`c901218`](https://github.com/AOSC-Dev/aoscdk-rs/commit/c9012182b87751171d88aa2d967849be44d9f500))
    - Bump version to 0.4 ([`14a4007`](https://github.com/AOSC-Dev/aoscdk-rs/commit/14a40076a484c8c4dbe0dd1f9f53169a6d33efba))
    - Fix some string ([`72fe58a`](https://github.com/AOSC-Dev/aoscdk-rs/commit/72fe58a795803be96c78cb68d387f5cbc478e472))
    - Revise UI strings ([`b6da46e`](https://github.com/AOSC-Dev/aoscdk-rs/commit/b6da46e09046a833309acb5001a7115729c3033e))
    - Use cargo clippy ([`ae8c9c5`](https://github.com/AOSC-Dev/aoscdk-rs/commit/ae8c9c5f24adfc9c979d00bf1d51f3e408bbaf4c))
    - Fix get lock ([`313b5ef`](https://github.com/AOSC-Dev/aoscdk-rs/commit/313b5efc93033fbab1272290d0ae1d71dfe82cf3))
    - Add process lock ([`64d3bd6`](https://github.com/AOSC-Dev/aoscdk-rs/commit/64d3bd6b8358f590eded85438a1e62f299af3476))
    - You are now a dungeon master! (thx @cth451) ([`1c7a0f1`](https://github.com/AOSC-Dev/aoscdk-rs/commit/1c7a0f1ef7706f27228cd847be06e0d0479625c4))
    - Fix print install finished ([`50625d7`](https://github.com/AOSC-Dev/aoscdk-rs/commit/50625d76669361c8e071344ec888b5102ae1d0d8))
    - Fix join thread failed ([`23753ea`](https://github.com/AOSC-Dev/aoscdk-rs/commit/23753ead7aba413bdc5407a3e27443f3afd130d4))
    - Cancel install after exit ([`2715d44`](https://github.com/AOSC-Dev/aoscdk-rs/commit/2715d44f198802fc26ed5b1a30b5929818688f1f))
    - Fix error handle ([`861f1ab`](https://github.com/AOSC-Dev/aoscdk-rs/commit/861f1abe3fe2421bd6bcb2ccea127be29664db6d))
    - Use cargo clippy ([`22fe212`](https://github.com/AOSC-Dev/aoscdk-rs/commit/22fe212d141c3014f2b562db03a1006a40b6b940))
    - Fix auto swap ([`8a18b88`](https://github.com/AOSC-Dev/aoscdk-rs/commit/8a18b889073e98d2b4a62375a1a87cff55c8163a))
    - Fix default mirror download ([`29db63b`](https://github.com/AOSC-Dev/aoscdk-rs/commit/29db63bf15bc1680600068a2a09ef393c8830af4))
    - Add some feature ([`fc11578`](https://github.com/AOSC-Dev/aoscdk-rs/commit/fc11578b8c32ec0a4ddac77b2506145b20c56f17))
    - New ([`8b613f2`](https://github.com/AOSC-Dev/aoscdk-rs/commit/8b613f22486fac4e08ca523b11aac08bf7048f1f))
    - Fix cancel failed bug ([`a99e553`](https://github.com/AOSC-Dev/aoscdk-rs/commit/a99e553f2cddd13d6294a824d4b8fdf1e0874dd0))
    - Start_install view cancel install after umount fs ([`978c568`](https://github.com/AOSC-Dev/aoscdk-rs/commit/978c56867221ffb678eddad409e102390802241c))
    - Cargo clippy ([`5f57036`](https://github.com/AOSC-Dev/aoscdk-rs/commit/5f570363fb246968c2dc1a585a29900bf1a34c58))
    - Start_install view add cancel button ([`6b4c509`](https://github.com/AOSC-Dev/aoscdk-rs/commit/6b4c509c30909c6809fd7fa0d34955bedda89791))
    - Improve select_swap UI logic ([`bf0e2db`](https://github.com/AOSC-Dev/aoscdk-rs/commit/bf0e2dbdc9e37af9b32f6fc7ad72be701d94e8fa))
    - Select_swap view add back and exit button ([`c650b2d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/c650b2d42492a159539b7901c7e9bca0620754fc))
    - Improve ui ([`d6aa855`](https://github.com/AOSC-Dev/aoscdk-rs/commit/d6aa855faa6e7e342dad8d4cc66e296ffe22bd05))
    - Fix set swapfile permission ([`122f690`](https://github.com/AOSC-Dev/aoscdk-rs/commit/122f69057418328d59da9c7aeb184b027c117eef))
    - Add select_swap view to create swapfile ([`eb73dc2`](https://github.com/AOSC-Dev/aoscdk-rs/commit/eb73dc20b116de867cda93976ac4e5d8ae24b615))
    - Add riscv64 architecture mapping ([`0ae6dcc`](https://github.com/AOSC-Dev/aoscdk-rs/commit/0ae6dcc7edeef0139c66eb0fdbc71c9f38cbd290))
    - (cargo-release) start next development iteration 0.3.7-alpha.0 ([`8d21ace`](https://github.com/AOSC-Dev/aoscdk-rs/commit/8d21ace4388063a14707796d4f4f368f7d43ba9e))
</details>

## v0.3.6 (2022-02-19)

<csr-id-86e7fb93117fc09b6023459f093d93942b7670eb/>
<csr-id-c2618c316a0a856f7183cd6cd19d2dd8894ed97b/>
<csr-id-bee166d2563a2399dc176f01ef0d2a585314978e/>
<csr-id-69525ca420ff20b957f25002b52c3afded7045a2/>
<csr-id-e34cdba1761d5b9027f6fab803c880be7615ef12/>
<csr-id-a511f8b0be000ed2c95441b40b6af77066215c54/>
<csr-id-c4daa8ee4a433e70c9a370b48bd6d84d8c9cde82/>
<csr-id-5e92b3c16d8494c21784e926a85cb462a06b4946/>
<csr-id-3948e418c8565f9e3290feb53b2d8a5c0788b735/>
<csr-id-31529575f1654ab5d3fb626a0a32825045591e0c/>
<csr-id-13d6d290fdf3854f976bc9c0f10950d1b59013a6/>
<csr-id-9b5aec04370c1315e13478e1aa77304f79927881/>
<csr-id-7d7f5f91ff237a28de74fba8a3ec7b494db7a05e/>
<csr-id-10999930ca9585c58c4715c4db458b0f76bfc519/>
<csr-id-d8f4ab0011e180e5e3a96f5409e3f79f129939ce/>

### Other

 - <csr-id-86e7fb93117fc09b6023459f093d93942b7670eb/> move marco local
 - <csr-id-c2618c316a0a856f7183cd6cd19d2dd8894ed97b/> improve logic anagin in select_partition
 - <csr-id-bee166d2563a2399dc176f01ef0d2a585314978e/> improve logic in select_partition
 - <csr-id-69525ca420ff20b957f25002b52c3afded7045a2/> fix button cancel in select_partition
 - <csr-id-e34cdba1761d5b9027f6fab803c880be7615ef12/> improve fstab_entries format
 - <csr-id-a511f8b0be000ed2c95441b40b6af77066215c54/> add "are use sure use this fs type" window
   - Also fix fstab_entries match fat32 bug
 - <csr-id-c4daa8ee4a433e70c9a370b48bd6d84d8c9cde82/> fill of fstab_entries fs type
 - <csr-id-5e92b3c16d8494c21784e926a85cb462a06b4946/> pin cursive version
 - <csr-id-3948e418c8565f9e3290feb53b2d8a5c0788b735/> use cargo fmt and cargo clippy
 - <csr-id-31529575f1654ab5d3fb626a0a32825045591e0c/> fix genfstab_to_file
 - <csr-id-13d6d290fdf3854f976bc9c0f10950d1b59013a6/> use cargo fmt
 - <csr-id-9b5aec04370c1315e13478e1aa77304f79927881/> set some function as non-debug only
 - <csr-id-7d7f5f91ff237a28de74fba8a3ec7b494db7a05e/> add genfstab feature
 - <csr-id-10999930ca9585c58c4715c4db458b0f76bfc519/> only allow use ext4 fs type on root
 - <csr-id-d8f4ab0011e180e5e3a96f5409e3f79f129939ce/> improve logic in execute_locale_gen

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release over the course of 13 calendar days.
 - 13 days passed between releases.
 - 15 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.6 ([`7835dc3`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7835dc38c3ae8566753ccde788d07fb53b16512e))
    - Move marco local ([`86e7fb9`](https://github.com/AOSC-Dev/aoscdk-rs/commit/86e7fb93117fc09b6023459f093d93942b7670eb))
    - Improve logic anagin in select_partition ([`c2618c3`](https://github.com/AOSC-Dev/aoscdk-rs/commit/c2618c316a0a856f7183cd6cd19d2dd8894ed97b))
    - Improve logic in select_partition ([`bee166d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/bee166d2563a2399dc176f01ef0d2a585314978e))
    - Fix button cancel in select_partition ([`69525ca`](https://github.com/AOSC-Dev/aoscdk-rs/commit/69525ca420ff20b957f25002b52c3afded7045a2))
    - Improve fstab_entries format ([`e34cdba`](https://github.com/AOSC-Dev/aoscdk-rs/commit/e34cdba1761d5b9027f6fab803c880be7615ef12))
    - Add "are use sure use this fs type" window ([`a511f8b`](https://github.com/AOSC-Dev/aoscdk-rs/commit/a511f8b0be000ed2c95441b40b6af77066215c54))
    - Fill of fstab_entries fs type ([`c4daa8e`](https://github.com/AOSC-Dev/aoscdk-rs/commit/c4daa8ee4a433e70c9a370b48bd6d84d8c9cde82))
    - Revert "disk: only allow use ext4 fs type on root" ([`fe1328f`](https://github.com/AOSC-Dev/aoscdk-rs/commit/fe1328f1bd98936898e4df87b88c376def45168d))
    - Pin cursive version ([`5e92b3c`](https://github.com/AOSC-Dev/aoscdk-rs/commit/5e92b3c16d8494c21784e926a85cb462a06b4946))
    - Use cargo fmt and cargo clippy ([`3948e41`](https://github.com/AOSC-Dev/aoscdk-rs/commit/3948e418c8565f9e3290feb53b2d8a5c0788b735))
    - Fix genfstab_to_file ([`3152957`](https://github.com/AOSC-Dev/aoscdk-rs/commit/31529575f1654ab5d3fb626a0a32825045591e0c))
    - Use cargo fmt ([`13d6d29`](https://github.com/AOSC-Dev/aoscdk-rs/commit/13d6d290fdf3854f976bc9c0f10950d1b59013a6))
    - Set some function as non-debug only ([`9b5aec0`](https://github.com/AOSC-Dev/aoscdk-rs/commit/9b5aec04370c1315e13478e1aa77304f79927881))
    - Add genfstab feature ([`7d7f5f9`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7d7f5f91ff237a28de74fba8a3ec7b494db7a05e))
    - Only allow use ext4 fs type on root ([`1099993`](https://github.com/AOSC-Dev/aoscdk-rs/commit/10999930ca9585c58c4715c4db458b0f76bfc519))
    - Improve logic in execute_locale_gen ([`d8f4ab0`](https://github.com/AOSC-Dev/aoscdk-rs/commit/d8f4ab0011e180e5e3a96f5409e3f79f129939ce))
    - (cargo-release) start next development iteration 0.3.6-alpha.0 ([`66f3b8d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/66f3b8da02904be00ff22b4c9e79fc34304957f4))
</details>

## v0.3.5 (2022-02-05)

<csr-id-6c9a9444ebabbbaa74aa7d74a0301c048baccbef/>
<csr-id-5e264b8d195a5b294883aa033a2f7beb2321965d/>
<csr-id-b42b6fa5304024648e71a2cb1bf898bac56badbd/>
<csr-id-7e6aae209fc1f6a2ab01d98cb347312969101563/>
<csr-id-57011e7033bce23109502bec1eaeb8378ecaeba1/>
<csr-id-1796ed1fefdc6c5c67e31149a9d68debcf291ba4/>
<csr-id-643b14a571e3115e3b36ebac449dd528c3d9cc5c/>
<csr-id-88137127bf93b92d3d60ab006af77557a3342de2/>
<csr-id-95e36d091ccf00227e59c71506a2c88a6cfa8348/>
<csr-id-75ee1ff1c031594fed41dd5a862b59ef25954738/>
<csr-id-b7661eec29b772f7d3b68f5bd35c537a65b38e11/>
<csr-id-7538ac3e940500bec90b324a93ac6b5bf624eb76/>
<csr-id-45d5a714dd9156a61a84df4ff601dbfcffcf4020/>

### Other

 - <csr-id-6c9a9444ebabbbaa74aa7d74a0301c048baccbef/> use cargo fmt
 - <csr-id-5e264b8d195a5b294883aa033a2f7beb2321965d/> fix write /etc/locale.gen again
 - <csr-id-b42b6fa5304024648e71a2cb1bf898bac56badbd/> fix parse comment bug
 - <csr-id-7e6aae209fc1f6a2ab01d98cb347312969101563/> fix write /etc/locale.gen
 - <csr-id-57011e7033bce23109502bec1eaeb8378ecaeba1/> improve test case
 - <csr-id-1796ed1fefdc6c5c67e31149a9d68debcf291ba4/> add test_download_i486 test case
 - <csr-id-643b14a571e3115e3b36ebac449dd528c3d9cc5c/> do not new thread to execute install::gen_ssh_key
   Because opening a new thread to run install::gen_ssh_key on a retro machine (e.g. Intel Pentium 3) can cause serious performance problems
 - <csr-id-88137127bf93b92d3d60ab006af77557a3342de2/> do not match comment in /etc/locale.gen
 - <csr-id-95e36d091ccf00227e59c71506a2c88a6cfa8348/> execute_locale_gen now use drop comment in /etc/locale.gen
   - Also fix add non-encode line to /etc/locale.gen bug
 - <csr-id-75ee1ff1c031594fed41dd5a862b59ef25954738/> lint code again
 - <csr-id-b7661eec29b772f7d3b68f5bd35c537a65b38e11/> improve logic in download_file function
 - <csr-id-7538ac3e940500bec90b324a93ac6b5bf624eb76/> add "8/8 Finalising installation ..." output
 - <csr-id-45d5a714dd9156a61a84df4ff601dbfcffcf4020/> add 7/7 thread sleep to fix cpu cycle used up in retro device

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 2 calendar days.
 - 2 days passed between releases.
 - 13 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.5 ([`ffd90b4`](https://github.com/AOSC-Dev/aoscdk-rs/commit/ffd90b458b30f19f5a2a9b0d88ab6f412e83693d))
    - Use cargo fmt ([`6c9a944`](https://github.com/AOSC-Dev/aoscdk-rs/commit/6c9a9444ebabbbaa74aa7d74a0301c048baccbef))
    - Fix write /etc/locale.gen again ([`5e264b8`](https://github.com/AOSC-Dev/aoscdk-rs/commit/5e264b8d195a5b294883aa033a2f7beb2321965d))
    - Fix parse comment bug ([`b42b6fa`](https://github.com/AOSC-Dev/aoscdk-rs/commit/b42b6fa5304024648e71a2cb1bf898bac56badbd))
    - Fix write /etc/locale.gen ([`7e6aae2`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7e6aae209fc1f6a2ab01d98cb347312969101563))
    - Improve test case ([`57011e7`](https://github.com/AOSC-Dev/aoscdk-rs/commit/57011e7033bce23109502bec1eaeb8378ecaeba1))
    - Add test_download_i486 test case ([`1796ed1`](https://github.com/AOSC-Dev/aoscdk-rs/commit/1796ed1fefdc6c5c67e31149a9d68debcf291ba4))
    - Do not new thread to execute install::gen_ssh_key ([`643b14a`](https://github.com/AOSC-Dev/aoscdk-rs/commit/643b14a571e3115e3b36ebac449dd528c3d9cc5c))
    - Do not match comment in /etc/locale.gen ([`8813712`](https://github.com/AOSC-Dev/aoscdk-rs/commit/88137127bf93b92d3d60ab006af77557a3342de2))
    - Execute_locale_gen now use drop comment in /etc/locale.gen ([`95e36d0`](https://github.com/AOSC-Dev/aoscdk-rs/commit/95e36d091ccf00227e59c71506a2c88a6cfa8348))
    - Lint code again ([`75ee1ff`](https://github.com/AOSC-Dev/aoscdk-rs/commit/75ee1ff1c031594fed41dd5a862b59ef25954738))
    - Improve logic in download_file function ([`b7661ee`](https://github.com/AOSC-Dev/aoscdk-rs/commit/b7661eec29b772f7d3b68f5bd35c537a65b38e11))
    - Add "8/8 Finalising installation ..." output ([`7538ac3`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7538ac3e940500bec90b324a93ac6b5bf624eb76))
    - Add 7/7 thread sleep to fix cpu cycle used up in retro device ([`45d5a71`](https://github.com/AOSC-Dev/aoscdk-rs/commit/45d5a714dd9156a61a84df4ff601dbfcffcf4020))
    - (cargo-release) start next development iteration 0.3.5-alpha.0 ([`ccc4a71`](https://github.com/AOSC-Dev/aoscdk-rs/commit/ccc4a714a6583656812571b69d190fd6ee613b94))
</details>

## v0.3.4 (2022-02-03)

<csr-id-e9abd4fea586ac1c399c8f5a90a180e056e44a11/>
<csr-id-1cab2eb5707c499df506c1f4d8c71db208e26c61/>
<csr-id-2699717ff5bc1434cbf6b0d57e204f6f34c3a55a/>
<csr-id-bb016d53b31ae3f7586aaff9f95c3593477f414d/>
<csr-id-932e18f6c5d8b267780f5839cecb429fdfd4c563/>
<csr-id-be18832360f59c4454852825b8fbe9a76161dbe9/>
<csr-id-e2424cbe2a95aba88d7d3003fa839320e79e5815/>
<csr-id-81a019e31ddd42f996ecec843705717163fa7707/>

### Other

 - <csr-id-e9abd4fea586ac1c399c8f5a90a180e056e44a11/> improve test case
 - <csr-id-1cab2eb5707c499df506c1f4d8c71db208e26c61/> fix handle tarball 404 error
 - <csr-id-2699717ff5bc1434cbf6b0d57e204f6f34c3a55a/> remove useless function
 - <csr-id-bb016d53b31ae3f7586aaff9f95c3593477f414d/> add bye_chroot function comment
 - <csr-id-932e18f6c5d8b267780f5839cecb429fdfd4c563/> set root_fd as lazy result
 - <csr-id-be18832360f59c4454852825b8fbe9a76161dbe9/> set umount_all function return None
 - <csr-id-e2424cbe2a95aba88d7d3003fa839320e79e5815/> bye_chroot not need to use pub
 - <csr-id-81a019e31ddd42f996ecec843705717163fa7707/> add bye_chroot function to if return error, umount_all before escape chroot

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 4 calendar days.
 - 4 days passed between releases.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.4 ([`36a0762`](https://github.com/AOSC-Dev/aoscdk-rs/commit/36a076243c1dde2391866b69d15739f7799e5a78))
    - Improve test case ([`e9abd4f`](https://github.com/AOSC-Dev/aoscdk-rs/commit/e9abd4fea586ac1c399c8f5a90a180e056e44a11))
    - Fix handle tarball 404 error ([`1cab2eb`](https://github.com/AOSC-Dev/aoscdk-rs/commit/1cab2eb5707c499df506c1f4d8c71db208e26c61))
    - Remove useless function ([`2699717`](https://github.com/AOSC-Dev/aoscdk-rs/commit/2699717ff5bc1434cbf6b0d57e204f6f34c3a55a))
    - Add bye_chroot function comment ([`bb016d5`](https://github.com/AOSC-Dev/aoscdk-rs/commit/bb016d53b31ae3f7586aaff9f95c3593477f414d))
    - Set root_fd as lazy result ([`932e18f`](https://github.com/AOSC-Dev/aoscdk-rs/commit/932e18f6c5d8b267780f5839cecb429fdfd4c563))
    - Set umount_all function return None ([`be18832`](https://github.com/AOSC-Dev/aoscdk-rs/commit/be18832360f59c4454852825b8fbe9a76161dbe9))
    - Bye_chroot not need to use pub ([`e2424cb`](https://github.com/AOSC-Dev/aoscdk-rs/commit/e2424cbe2a95aba88d7d3003fa839320e79e5815))
    - Add bye_chroot function to if return error, umount_all before escape chroot ([`81a019e`](https://github.com/AOSC-Dev/aoscdk-rs/commit/81a019e31ddd42f996ecec843705717163fa7707))
    - (cargo-release) start next development iteration 0.3.4-alpha.0 ([`4e19c4a`](https://github.com/AOSC-Dev/aoscdk-rs/commit/4e19c4a6a092f79b60b56d942e50ccfc7560d482))
</details>

## v0.3.3 (2022-01-29)

<csr-id-00e937bb9da63eced65c775f57c27ec4ee39c19a/>

### Other

 - <csr-id-00e937bb9da63eced65c775f57c27ec4ee39c19a/> fix 7/7 output

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.3 ([`cbd4a1c`](https://github.com/AOSC-Dev/aoscdk-rs/commit/cbd4a1cbf5828e6e00e719d6f32990eb9866434a))
    - Fix 7/7 output ([`00e937b`](https://github.com/AOSC-Dev/aoscdk-rs/commit/00e937bb9da63eced65c775f57c27ec4ee39c19a))
    - (cargo-release) start next development iteration 0.3.3-alpha.0 ([`77bf276`](https://github.com/AOSC-Dev/aoscdk-rs/commit/77bf276cef07c7bcc348633a952abc8b6d77f645))
</details>

## v0.3.2 (2022-01-27)

<csr-id-f963c149fde658b66e715f6d0c5848db5dd0893e/>
<csr-id-b4721b16fd333a82f30e11e570a10811581cee99/>
<csr-id-3f91809fbc0307c44401d2f6b06b6f28abed112d/>
<csr-id-35aa5bc357be87e7c9e9867df1cb7602a87f2de0/>

### Other

 - <csr-id-f963c149fde658b66e715f6d0c5848db5dd0893e/> set version as 0.3.2-alpha.0 to make cargo-release happy
 - <csr-id-b4721b16fd333a82f30e11e570a10811581cee99/> fix gen_ssh_key command error message typo
 - <csr-id-3f91809fbc0307c44401d2f6b06b6f28abed112d/> fix gen_ssh_key command typo
 - <csr-id-35aa5bc357be87e7c9e9867df1cb7602a87f2de0/> fix a typo

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.2 ([`312eb17`](https://github.com/AOSC-Dev/aoscdk-rs/commit/312eb1701a9fb61d446eb500ae7f69f7bc971f0a))
    - Set version as 0.3.2-alpha.0 to make cargo-release happy ([`f963c14`](https://github.com/AOSC-Dev/aoscdk-rs/commit/f963c149fde658b66e715f6d0c5848db5dd0893e))
    - Fix gen_ssh_key command error message typo ([`b4721b1`](https://github.com/AOSC-Dev/aoscdk-rs/commit/b4721b16fd333a82f30e11e570a10811581cee99))
    - Fix gen_ssh_key command typo ([`3f91809`](https://github.com/AOSC-Dev/aoscdk-rs/commit/3f91809fbc0307c44401d2f6b06b6f28abed112d))
    - Fix a typo ([`35aa5bc`](https://github.com/AOSC-Dev/aoscdk-rs/commit/35aa5bc357be87e7c9e9867df1cb7602a87f2de0))
    - Install :add gen_ssh_key for retro machine ([`d6e911d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/d6e911d8e9b4d43ebfe717aa78b33cadc2ea0525))
</details>

## v0.3.1 (2022-01-27)

<csr-id-18ba0f55f9093859eaf5e18c356045d2427015b4/>
<csr-id-5393c1fe81ae4ab8b1bf3a02475c9def2f1dc5b6/>
<csr-id-8ee1b488c78ee9e2857ba1197e406608686565c6/>
<csr-id-d70d505183393a6ab00812ed0838bb738099a1d7/>
<csr-id-c504e1187fec493690ae6848432482b5cb7850c6/>
<csr-id-71a8e1bfdd3c3cb8abfe42871696e679018bf380/>
<csr-id-652fb9f63083d45f14343f823f0af7cea532b309/>

### Other

 - <csr-id-18ba0f55f9093859eaf5e18c356045d2427015b4/> fix show finish after unwrap error failed
 - <csr-id-5393c1fe81ae4ab8b1bf3a02475c9def2f1dc5b6/> use map_err to lint code
   - Also use cargo cliipy
 - <csr-id-8ee1b488c78ee9e2857ba1197e406608686565c6/> better error info display in network send error
 - <csr-id-d70d505183393a6ab00812ed0838bb738099a1d7/> try to solve network redirect connect error
 - <csr-id-c504e1187fec493690ae6848432482b5cb7850c6/> Set UTC/RTC => RTC Timezone
 - <csr-id-71a8e1bfdd3c3cb8abfe42871696e679018bf380/> improve installation progress message
 - <csr-id-652fb9f63083d45f14343f823f0af7cea532b309/> bump version to 0.3.1-alpha.0 to make cargo release happy

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 17 calendar days.
 - 17 days passed between releases.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.1 ([`77a867e`](https://github.com/AOSC-Dev/aoscdk-rs/commit/77a867e4b0e99ce48dd6f01ec419b291562af43c))
    - Fix show finish after unwrap error failed ([`18ba0f5`](https://github.com/AOSC-Dev/aoscdk-rs/commit/18ba0f55f9093859eaf5e18c356045d2427015b4))
    - Use map_err to lint code ([`5393c1f`](https://github.com/AOSC-Dev/aoscdk-rs/commit/5393c1fe81ae4ab8b1bf3a02475c9def2f1dc5b6))
    - Better error info display in network send error ([`8ee1b48`](https://github.com/AOSC-Dev/aoscdk-rs/commit/8ee1b488c78ee9e2857ba1197e406608686565c6))
    - Try to solve network redirect connect error ([`d70d505`](https://github.com/AOSC-Dev/aoscdk-rs/commit/d70d505183393a6ab00812ed0838bb738099a1d7))
    - Set UTC/RTC => RTC Timezone ([`c504e11`](https://github.com/AOSC-Dev/aoscdk-rs/commit/c504e1187fec493690ae6848432482b5cb7850c6))
    - Improve installation progress message ([`71a8e1b`](https://github.com/AOSC-Dev/aoscdk-rs/commit/71a8e1bfdd3c3cb8abfe42871696e679018bf380))
    - Bump version to 0.3.1-alpha.0 to make cargo release happy ([`652fb9f`](https://github.com/AOSC-Dev/aoscdk-rs/commit/652fb9f63083d45f14343f823f0af7cea532b309))
</details>

## v0.3.0 (2022-01-11)

<csr-id-0055597d756ba666a62ec482526c4136328b6e22/>
<csr-id-f3ecd6ac31c88a92ed21667111bac0811fea34d2/>
<csr-id-2b484978c1ba0c94d9e7f9285d890db8ddbe2096/>
<csr-id-487207936e2ceddbae94c37e98cb0a8d5413c174/>
<csr-id-b60c3406326a18721642b03d1600455b6527bda0/>
<csr-id-d0368c3efc6d73315312f443789e16912ac6ef80/>
<csr-id-5cc7a285792d085cd4ab4118f001fe04e57028f6/>
<csr-id-1324c10db1144f24e6e2c2cc636ad0b548b4e7f6/>
<csr-id-26d45d661730f15dedb94d6fc187de02e2fa133d/>
<csr-id-2b938c06a9b7cbc10f45d14911ceb1e2d67cd146/>
<csr-id-4c6fd9fe12a717533e80945f50c06314213f05de/>
<csr-id-5cf221287ef37ea13f712556631c2077c636f68c/>
<csr-id-440517c47f41992b6c2e233f386964075128b8fb/>
<csr-id-95da7266b03bd86c42794cc697f55c7fbfc27215/>
<csr-id-aed8db271cb5a4d806623f21cb03f61a2cc3adc3/>
<csr-id-8bf89397792274e6410e663f31220d7378ada860/>
<csr-id-37f28d0a382ed31df2a525c1adfd4aba1457d2a7/>
<csr-id-68928f6a4d80a3fe8e4058234a465a8f2e16b079/>
<csr-id-5bf5b8262475cf0e495c58db4a22742f736342e7/>
<csr-id-f834fa6d8e187cb2c26d0c1091cb8714e93ec98a/>
<csr-id-9adbbfc1f58b1cf262304469e505c003e0e70afd/>
<csr-id-cbb2e4329adb95aa0ca7cdc796cc84a932a87429/>
<csr-id-d10207ee77d4ba1fae6531b16cf0fe01e4123681/>
<csr-id-803170788208e4f8e3189a9029d2b258a5944098/>
<csr-id-3c884b90c420a973966ba4266b7ccd234c543f90/>
<csr-id-192a905a9632a5ec28b5f65480c450fdcdb244c6/>
<csr-id-2f1521dfd3c25b6d8a67ceeab5a08ad682870c5e/>
<csr-id-7a4378571aee6ea52cba419bdcc0deaa973a8cd5/>
<csr-id-7c32c8c958073c3b10e019d0babd40461f331256/>
<csr-id-19144d6983d3c62c7331d130e57647694f53b08c/>
<csr-id-35c67070105d2ff09748cdbc497161d9f471c8bc/>
<csr-id-b8f908aaf7a6ab9428d20fae6e3b39a2270cf590/>
<csr-id-09a137c8739775c1c994802e63fec5a74fb60abd/>
<csr-id-565db1551de4dcd9745ed89e9f88d6f7e2b19ad5/>
<csr-id-fd66b1772548e907ba11374d6a35fc2dd804966e/>
<csr-id-7c8e6a8d245790a44db975ca2b430d1a144c0dbb/>
<csr-id-f926947028a9cb030514a47850974bab9a785b6e/>
<csr-id-ef2ec451cdce30384f6acc9f88ade47bef381a95/>
<csr-id-887df4a4f2ed73b72da6478a6777646550ce89f7/>
<csr-id-76fe0a2ecc6e8e768d7a8497a1219837808699f1/>
<csr-id-fb3569b0822ddcdca0d6d14eafe0bc2c2bc33a2e/>
<csr-id-1bb8a05a825426961cc9bb59ff6bcb8faa0e45e0/>
<csr-id-4d8b8438ccad4e5be5d6a8a0c6da62a0cba437d4/>
<csr-id-2ccea3bcdef557a67d8622e0425cb8a1e1863c1f/>
<csr-id-bbca219a3c119c92e841029e179dc12945e3a45a/>
<csr-id-59d5ec806733890a74307c29dc589b50c651ae62/>
<csr-id-8b526429e3d5ab079feea277e0cff01d0999a189/>
<csr-id-622512d0d0c46c7c63cdf523e4fa1d0924c8aa08/>
<csr-id-ed1e3e51f8a803ae16c9e2ddd0a9a3a8e949dd20/>
<csr-id-cf81cbea58a7fcab59a826cdc9de2f35ad865e52/>
<csr-id-875b41144eb1644eacd5170ae45903c08982b457/>
<csr-id-8836a2b15e24327e8c1f7fe3c7d9d7268ca37a08/>
<csr-id-9ece6ed5c791dea6442ab89cb8b60da6feb40bc8/>
<csr-id-d775a5e8caa95802d16f7c4ab74ce032752d859d/>
<csr-id-90b68dfc68d527e27baa9c2d1fd545b8981a4c8e/>
<csr-id-7f5da7eadc105670dec92929bc8d86ba303b7f16/>
<csr-id-fed5db25c99a4b5834b085207e265d04397bf293/>
<csr-id-7cad2c982f6fa3aede479b0185da1233fa420801/>
<csr-id-b555dc87757a51a2e3d8e7155dd9910fadd50f2e/>
<csr-id-4e3d623a3643d7764f03b522e55d3a1bdaab3ba6/>
<csr-id-cabda31fd4848fe2d466e7ef74f829a6338c4895/>
<csr-id-30ec9bfbb8215c7de7bc3477ba8f480b15c0f976/>
<csr-id-c2a3f65e82b6fa4c57644d632cb355f3d4cd2c61/>
<csr-id-67dad60ab26498b0a9214f31f0814fb4bfec8113/>
<csr-id-e82d6388b424c929beec6d47981682e8e4fba7a3/>
<csr-id-618fb1cc8a25e5c843d229b07649ddc0b78b62df/>
<csr-id-2e3d93fde08418e59a7403ed2fc9eb7707beaf4c/>
<csr-id-558eacca25e5f7084bd4909711c3b0e780637280/>
<csr-id-7ec2324028f6b2e14643ef9392d5a6e0c2cdc4a9/>
<csr-id-b0ab3d4f3abb1ae9f130da9c495d18f2a6df4beb/>
<csr-id-530f69a88268c70195412e2ab72344c85382b09a/>
<csr-id-ef4ccfe22be5648766340cc10713afd2bc004c13/>
<csr-id-a945c8a5d1b88a34da3ff3df44c7ed35bd8acc3b/>
<csr-id-d6272c5d2e738526b52a140f95179e877ee6ff58/>
<csr-id-f95ae86380fd22911a4bd1ec4b2f4b42223cf460/>
<csr-id-8b86b9559a3aa90eb77344c7b733e61bee154620/>
<csr-id-a063635b9b8e33ec1e92077bd5054b4a12405540/>
<csr-id-bc1bf0d168e9c7e186108da0c3defeee993d278a/>
<csr-id-ab7b14a1a9955c9ebe1c572768f5828a8b2944d9/>
<csr-id-b9316a1f2e21fa47e142f2a3f3c79e134cf81dd0/>
<csr-id-2dbe6531d3c76659ad3de48227b3a28d0247b38a/>
<csr-id-5531b4f18fab0c5995750f54e975f3b5364f674e/>

### Other

 - <csr-id-0055597d756ba666a62ec482526c4136328b6e22/> bump version to 0.3
 - <csr-id-f3ecd6ac31c88a92ed21667111bac0811fea34d2/> add English translation
 - <csr-id-2b484978c1ba0c94d9e7f9285d890db8ddbe2096/> drop an unneeded sudo
 - <csr-id-487207936e2ceddbae94c37e98cb0a8d5413c174/> grammatical (???) lint
 - <csr-id-b60c3406326a18721642b03d1600455b6527bda0/> fix typo
 - <csr-id-d0368c3efc6d73315312f443789e16912ac6ef80/> add zh README
 - <csr-id-5cc7a285792d085cd4ab4118f001fe04e57028f6/> set some button text
 - <csr-id-1324c10db1144f24e6e2c2cc636ad0b548b4e7f6/> fix a typo
 - <csr-id-26d45d661730f15dedb94d6fc187de02e2fa133d/> use cargo clippy and cargo fmt
 - <csr-id-2b938c06a9b7cbc10f45d14911ceb1e2d67cd146/> not allow user ctrl-c exit in start_install()
 - <csr-id-4c6fd9fe12a717533e80945f50c06314213f05de/> fix umount_all() but this only under non-chroot steps
 - <csr-id-5cf221287ef37ea13f712556631c2077c636f68c/> add "Timezone selected" layout
 - <csr-id-440517c47f41992b6c2e233f386964075128b8fb/> use cargo clippy
   - Also lint code by myself
 - <csr-id-95da7266b03bd86c42794cc697f55c7fbfc27215/> fix install counter multiply overflow on retro
 - <csr-id-aed8db271cb5a4d806623f21cb03f61a2cc3adc3/> pin cursive-table-view version
 - <csr-id-8bf89397792274e6410e663f31220d7378ada860/> add execute_locale_gen to make retro user happy
 - <csr-id-37f28d0a382ed31df2a525c1adfd4aba1457d2a7/> use liushuyu/tar-rs fork to set perserve ownerships
 - <csr-id-68928f6a4d80a3fe8e4058234a465a8f2e16b079/> return error if all tarball list is empty
 - <csr-id-5bf5b8262475cf0e495c58db4a22742f736342e7/> fill partition info in test_dowmload()
 - <csr-id-f834fa6d8e187cb2c26d0c1091cb8714e93ec98a/> set debug disk size as required_size to easy install
 - <csr-id-9adbbfc1f58b1cf262304469e505c003e0e70afd/> continue if some recipe.tarballs is empty
 - <csr-id-cbb2e4329adb95aa0ca7cdc796cc84a932a87429/> use cargo fmt and clippy to lint code
 - <csr-id-d10207ee77d4ba1fae6531b16cf0fe01e4123681/> add fake counter
 - <csr-id-803170788208e4f8e3189a9029d2b258a5944098/> add "Verifying system release ..." view
 - <csr-id-3c884b90c420a973966ba4266b7ccd234c543f90/> better show_finished() output
 - <csr-id-192a905a9632a5ec28b5f65480c450fdcdb244c6/> Extracting system release => Unpacking system release
 - <csr-id-2f1521dfd3c25b6d8a67ceeab5a08ad682870c5e/> sha256_work hasher not need clone
 - <csr-id-7a4378571aee6ea52cba419bdcc0deaa973a8cd5/> better error handling
 - <csr-id-7c32c8c958073c3b10e019d0babd40461f331256/> better error output for fallocate
 - <csr-id-19144d6983d3c62c7331d130e57647694f53b08c/> use nix::fcntl::fallocate instead of fs3::allocate
   Originally, I thought fs3::allocate could handle errors better, but later I found that fs3::allocate does not set errmo using libc::posix_fallocate, while libc::fallocate does, and libc::fallocate is suitable for such a scenario.
 - <csr-id-35c67070105d2ff09748cdbc497161d9f471c8bc/> set user-agent as AOSC Deploykit/VERSION
 - <csr-id-b8f908aaf7a6ab9428d20fae6e3b39a2270cf590/> downgrade to rust 2018 edition to fix AOSC OS/Retro build
   The 2021 edition is not currently supported by Rust in the AOSC OS/Retro repository
 - <csr-id-09a137c8739775c1c994802e63fec5a74fb60abd/> improve output for UTC/RTC settings
 - <csr-id-565db1551de4dcd9745ed89e9f88d6f7e2b19ad5/> use const str to write some msg
 - <csr-id-fd66b1772548e907ba11374d6a35fc2dd804966e/> add benchmark mirrors dialog
 - <csr-id-7c8e6a8d245790a44db975ca2b430d1a144c0dbb/> set debug partition size as 4284067840 to fix base tarball install
   - Also better if current_partition.size < required_size output msg
 - <csr-id-f926947028a9cb030514a47850974bab9a785b6e/> adjust button order
 - <csr-id-ef2ec451cdce30384f6acc9f88ade47bef381a95/> If there is not enough disk space, prevent the user from continuing the installation
 - <csr-id-887df4a4f2ed73b72da6478a6777646550ce89f7/> use rust 2021 edition
 - <csr-id-76fe0a2ecc6e8e768d7a8497a1219837808699f1/> cargo clippy and fmt
 - <csr-id-fb3569b0822ddcdca0d6d14eafe0bc2c2bc33a2e/> parallel get_mirror_speed_score()
   - Also lint code and remove not used code
 - <csr-id-1bb8a05a825426961cc9bb59ff6bcb8faa0e45e0/> if install have error, umount fs
 - <csr-id-4d8b8438ccad4e5be5d6a8a0c6da62a0cba437d4/> set locale and RTC/UTC default
 - <csr-id-2ccea3bcdef557a67d8622e0425cb8a1e1863c1f/> add back and exit button
 - <csr-id-bbca219a3c119c92e841029e179dc12945e3a45a/> use marco to show "Please fill in all the fields" message
 - <csr-id-59d5ec806733890a74307c29dc589b50c651ae62/> full user info of show_summery()
 - <csr-id-8b526429e3d5ab079feea277e0cff01d0999a189/> split select_user()
 - <csr-id-622512d0d0c46c7c63cdf523e4fa1d0924c8aa08/> better layout
 - <csr-id-ed1e3e51f8a803ae16c9e2ddd0a9a3a8e949dd20/> fix set rtc localtime
 - <csr-id-cf81cbea58a7fcab59a826cdc9de2f35ad865e52/> better select mirrors view
 - <csr-id-875b41144eb1644eacd5170ae45903c08982b457/> better variant view
 - <csr-id-8836a2b15e24327e8c1f7fe3c7d9d7268ca37a08/> better welcome page
 - <csr-id-9ece6ed5c791dea6442ab89cb8b60da6feb40bc8/> fix set locale failed
 - <csr-id-d775a5e8caa95802d16f7c4ab74ce032752d859d/> update install test
 - <csr-id-90b68dfc68d527e27baa9c2d1fd545b8981a4c8e/> add zone1970 parser test
   - Also lint tui.rs code
 - <csr-id-7f5da7eadc105670dec92929bc8d86ba303b7f16/> select_variant before pop_layer
 - <csr-id-fed5db25c99a4b5834b085207e265d04397bf293/> add mirror list speedtest feature
 - <csr-id-7cad2c982f6fa3aede479b0185da1233fa420801/> use fs3 to allocate file
 - <csr-id-b555dc87757a51a2e3d8e7155dd9910fadd50f2e/> allow user select city view pop layout re-select continent
 - <csr-id-4e3d623a3643d7764f03b522e55d3a1bdaab3ba6/> better ux logic
 - <csr-id-cabda31fd4848fe2d466e7ef74f829a6338c4895/> add set utc/rtc option
 - <csr-id-30ec9bfbb8215c7de7bc3477ba8f480b15c0f976/> remove /etc/localtime after symlink timezone
 - <csr-id-c2a3f65e82b6fa4c57644d632cb355f3d4cd2c61/> better set timezone ux
 - <csr-id-67dad60ab26498b0a9214f31f0814fb4bfec8113/> add zone select
 - <csr-id-e82d6388b424c929beec6d47981682e8e4fba7a3/> add set_hwclock_tc() function
 - <csr-id-618fb1cc8a25e5c843d229b07649ddc0b78b62df/> add zone1970.tab parser
 - <csr-id-2e3d93fde08418e59a7403ed2fc9eb7707beaf4c/> should not return an error when remove_file() fails, since it is already the last step
 - <csr-id-558eacca25e5f7084bd4909711c3b0e780637280/> add more err check
 - <csr-id-7ec2324028f6b2e14643ef9392d5a6e0c2cdc4a9/> add send_error marco to lint code
 - <csr-id-b0ab3d4f3abb1ae9f130da9c495d18f2a6df4beb/> add some error check
 - <csr-id-530f69a88268c70195412e2ab72344c85382b09a/> fallocate before download
 - <csr-id-ef4ccfe22be5648766340cc10713afd2bc004c13/> add write_all error handle
 - <csr-id-a945c8a5d1b88a34da3ff3df44c7ed35bd8acc3b/> use threads to download tarball and checksum
 - <csr-id-d6272c5d2e738526b52a140f95179e877ee6ff58/> add download tarball error handle more message
 - <csr-id-f95ae86380fd22911a4bd1ec4b2f4b42223cf460/> add sha256sum match
 - <csr-id-8b86b9559a3aa90eb77344c7b733e61bee154620/> no save of partition configuration
   Since it is not sure if the partition number selected by the user is always the same, it is changed to not save the partition selected by the user
   - Also show_summery() add 'save config' feature
   - Use serde_json instead of toml
 - <csr-id-a063635b9b8e33ec1e92077bd5054b4a12405540/> save user config to next time run deploykit read config
 - <csr-id-bc1bf0d168e9c7e186108da0c3defeee993d278a/> if failed to download tarball, return error
 - <csr-id-ab7b14a1a9955c9ebe1c572768f5828a8b2944d9/> update dependencies
 - <csr-id-b9316a1f2e21fa47e142f2a3f3c79e134cf81dd0/> do not allow user install buildkit tarball
 - <csr-id-2dbe6531d3c76659ad3de48227b3a28d0247b38a/> make executing gparted non-blocking
 - <csr-id-5531b4f18fab0c5995750f54e975f3b5364f674e/> add better support for retro mode ...
   ... now when DeployKit is compiled with `--features is_retro`, dracut
   will not be run and retro variant tarballs will be selected

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 84 commits contributed to the release over the course of 212 calendar days.
 - 212 days passed between releases.
 - 82 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 ([`7c8cf7c`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7c8cf7cfc03f386b9d904ccabe23399fe0b18676))
    - Bump version to 0.3 ([`0055597`](https://github.com/AOSC-Dev/aoscdk-rs/commit/0055597d756ba666a62ec482526c4136328b6e22))
    - Add English translation ([`f3ecd6a`](https://github.com/AOSC-Dev/aoscdk-rs/commit/f3ecd6ac31c88a92ed21667111bac0811fea34d2))
    - Drop an unneeded sudo ([`2b48497`](https://github.com/AOSC-Dev/aoscdk-rs/commit/2b484978c1ba0c94d9e7f9285d890db8ddbe2096))
    - Grammatical (???) lint ([`4872079`](https://github.com/AOSC-Dev/aoscdk-rs/commit/487207936e2ceddbae94c37e98cb0a8d5413c174))
    - Fix typo ([`b60c340`](https://github.com/AOSC-Dev/aoscdk-rs/commit/b60c3406326a18721642b03d1600455b6527bda0))
    - Add zh README ([`d0368c3`](https://github.com/AOSC-Dev/aoscdk-rs/commit/d0368c3efc6d73315312f443789e16912ac6ef80))
    - Set some button text ([`5cc7a28`](https://github.com/AOSC-Dev/aoscdk-rs/commit/5cc7a285792d085cd4ab4118f001fe04e57028f6))
    - Fix a typo ([`1324c10`](https://github.com/AOSC-Dev/aoscdk-rs/commit/1324c10db1144f24e6e2c2cc636ad0b548b4e7f6))
    - Use cargo clippy and cargo fmt ([`26d45d6`](https://github.com/AOSC-Dev/aoscdk-rs/commit/26d45d661730f15dedb94d6fc187de02e2fa133d))
    - Not allow user ctrl-c exit in start_install() ([`2b938c0`](https://github.com/AOSC-Dev/aoscdk-rs/commit/2b938c06a9b7cbc10f45d14911ceb1e2d67cd146))
    - Fix umount_all() but this only under non-chroot steps ([`4c6fd9f`](https://github.com/AOSC-Dev/aoscdk-rs/commit/4c6fd9fe12a717533e80945f50c06314213f05de))
    - Add "Timezone selected" layout ([`5cf2212`](https://github.com/AOSC-Dev/aoscdk-rs/commit/5cf221287ef37ea13f712556631c2077c636f68c))
    - Use cargo clippy ([`440517c`](https://github.com/AOSC-Dev/aoscdk-rs/commit/440517c47f41992b6c2e233f386964075128b8fb))
    - Fix install counter multiply overflow on retro ([`95da726`](https://github.com/AOSC-Dev/aoscdk-rs/commit/95da7266b03bd86c42794cc697f55c7fbfc27215))
    - Pin cursive-table-view version ([`aed8db2`](https://github.com/AOSC-Dev/aoscdk-rs/commit/aed8db271cb5a4d806623f21cb03f61a2cc3adc3))
    - Add execute_locale_gen to make retro user happy ([`8bf8939`](https://github.com/AOSC-Dev/aoscdk-rs/commit/8bf89397792274e6410e663f31220d7378ada860))
    - Use liushuyu/tar-rs fork to set perserve ownerships ([`37f28d0`](https://github.com/AOSC-Dev/aoscdk-rs/commit/37f28d0a382ed31df2a525c1adfd4aba1457d2a7))
    - Return error if all tarball list is empty ([`68928f6`](https://github.com/AOSC-Dev/aoscdk-rs/commit/68928f6a4d80a3fe8e4058234a465a8f2e16b079))
    - Fill partition info in test_dowmload() ([`5bf5b82`](https://github.com/AOSC-Dev/aoscdk-rs/commit/5bf5b8262475cf0e495c58db4a22742f736342e7))
    - Set debug disk size as required_size to easy install ([`f834fa6`](https://github.com/AOSC-Dev/aoscdk-rs/commit/f834fa6d8e187cb2c26d0c1091cb8714e93ec98a))
    - Continue if some recipe.tarballs is empty ([`9adbbfc`](https://github.com/AOSC-Dev/aoscdk-rs/commit/9adbbfc1f58b1cf262304469e505c003e0e70afd))
    - Use cargo fmt and clippy to lint code ([`cbb2e43`](https://github.com/AOSC-Dev/aoscdk-rs/commit/cbb2e4329adb95aa0ca7cdc796cc84a932a87429))
    - Add fake counter ([`d10207e`](https://github.com/AOSC-Dev/aoscdk-rs/commit/d10207ee77d4ba1fae6531b16cf0fe01e4123681))
    - Add "Verifying system release ..." view ([`8031707`](https://github.com/AOSC-Dev/aoscdk-rs/commit/803170788208e4f8e3189a9029d2b258a5944098))
    - Better show_finished() output ([`3c884b9`](https://github.com/AOSC-Dev/aoscdk-rs/commit/3c884b90c420a973966ba4266b7ccd234c543f90))
    - Extracting system release => Unpacking system release ([`192a905`](https://github.com/AOSC-Dev/aoscdk-rs/commit/192a905a9632a5ec28b5f65480c450fdcdb244c6))
    - Sha256_work hasher not need clone ([`2f1521d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/2f1521dfd3c25b6d8a67ceeab5a08ad682870c5e))
    - Better error handling ([`7a43785`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7a4378571aee6ea52cba419bdcc0deaa973a8cd5))
    - Better error output for fallocate ([`7c32c8c`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7c32c8c958073c3b10e019d0babd40461f331256))
    - Use nix::fcntl::fallocate instead of fs3::allocate ([`19144d6`](https://github.com/AOSC-Dev/aoscdk-rs/commit/19144d6983d3c62c7331d130e57647694f53b08c))
    - Set user-agent as AOSC Deploykit/VERSION ([`35c6707`](https://github.com/AOSC-Dev/aoscdk-rs/commit/35c67070105d2ff09748cdbc497161d9f471c8bc))
    - Downgrade to rust 2018 edition to fix AOSC OS/Retro build ([`b8f908a`](https://github.com/AOSC-Dev/aoscdk-rs/commit/b8f908aaf7a6ab9428d20fae6e3b39a2270cf590))
    - Improve output for UTC/RTC settings ([`09a137c`](https://github.com/AOSC-Dev/aoscdk-rs/commit/09a137c8739775c1c994802e63fec5a74fb60abd))
    - Use const str to write some msg ([`565db15`](https://github.com/AOSC-Dev/aoscdk-rs/commit/565db1551de4dcd9745ed89e9f88d6f7e2b19ad5))
    - Add benchmark mirrors dialog ([`fd66b17`](https://github.com/AOSC-Dev/aoscdk-rs/commit/fd66b1772548e907ba11374d6a35fc2dd804966e))
    - Set debug partition size as 4284067840 to fix base tarball install ([`7c8e6a8`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7c8e6a8d245790a44db975ca2b430d1a144c0dbb))
    - Adjust button order ([`f926947`](https://github.com/AOSC-Dev/aoscdk-rs/commit/f926947028a9cb030514a47850974bab9a785b6e))
    - If there is not enough disk space, prevent the user from continuing the installation ([`ef2ec45`](https://github.com/AOSC-Dev/aoscdk-rs/commit/ef2ec451cdce30384f6acc9f88ade47bef381a95))
    - Use rust 2021 edition ([`887df4a`](https://github.com/AOSC-Dev/aoscdk-rs/commit/887df4a4f2ed73b72da6478a6777646550ce89f7))
    - Cargo clippy and fmt ([`76fe0a2`](https://github.com/AOSC-Dev/aoscdk-rs/commit/76fe0a2ecc6e8e768d7a8497a1219837808699f1))
    - Parallel get_mirror_speed_score() ([`fb3569b`](https://github.com/AOSC-Dev/aoscdk-rs/commit/fb3569b0822ddcdca0d6d14eafe0bc2c2bc33a2e))
    - If install have error, umount fs ([`1bb8a05`](https://github.com/AOSC-Dev/aoscdk-rs/commit/1bb8a05a825426961cc9bb59ff6bcb8faa0e45e0))
    - Set locale and RTC/UTC default ([`4d8b843`](https://github.com/AOSC-Dev/aoscdk-rs/commit/4d8b8438ccad4e5be5d6a8a0c6da62a0cba437d4))
    - Add back and exit button ([`2ccea3b`](https://github.com/AOSC-Dev/aoscdk-rs/commit/2ccea3bcdef557a67d8622e0425cb8a1e1863c1f))
    - Use marco to show "Please fill in all the fields" message ([`bbca219`](https://github.com/AOSC-Dev/aoscdk-rs/commit/bbca219a3c119c92e841029e179dc12945e3a45a))
    - Full user info of show_summery() ([`59d5ec8`](https://github.com/AOSC-Dev/aoscdk-rs/commit/59d5ec806733890a74307c29dc589b50c651ae62))
    - Split select_user() ([`8b52642`](https://github.com/AOSC-Dev/aoscdk-rs/commit/8b526429e3d5ab079feea277e0cff01d0999a189))
    - Better layout ([`622512d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/622512d0d0c46c7c63cdf523e4fa1d0924c8aa08))
    - Fix set rtc localtime ([`ed1e3e5`](https://github.com/AOSC-Dev/aoscdk-rs/commit/ed1e3e51f8a803ae16c9e2ddd0a9a3a8e949dd20))
    - Better select mirrors view ([`cf81cbe`](https://github.com/AOSC-Dev/aoscdk-rs/commit/cf81cbea58a7fcab59a826cdc9de2f35ad865e52))
    - Better variant view ([`875b411`](https://github.com/AOSC-Dev/aoscdk-rs/commit/875b41144eb1644eacd5170ae45903c08982b457))
    - Better welcome page ([`8836a2b`](https://github.com/AOSC-Dev/aoscdk-rs/commit/8836a2b15e24327e8c1f7fe3c7d9d7268ca37a08))
    - Fix set locale failed ([`9ece6ed`](https://github.com/AOSC-Dev/aoscdk-rs/commit/9ece6ed5c791dea6442ab89cb8b60da6feb40bc8))
    - Update install test ([`d775a5e`](https://github.com/AOSC-Dev/aoscdk-rs/commit/d775a5e8caa95802d16f7c4ab74ce032752d859d))
    - Add zone1970 parser test ([`90b68df`](https://github.com/AOSC-Dev/aoscdk-rs/commit/90b68dfc68d527e27baa9c2d1fd545b8981a4c8e))
    - Select_variant before pop_layer ([`7f5da7e`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7f5da7eadc105670dec92929bc8d86ba303b7f16))
    - Add mirror list speedtest feature ([`fed5db2`](https://github.com/AOSC-Dev/aoscdk-rs/commit/fed5db25c99a4b5834b085207e265d04397bf293))
    - Use fs3 to allocate file ([`7cad2c9`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7cad2c982f6fa3aede479b0185da1233fa420801))
    - Allow user select city view pop layout re-select continent ([`b555dc8`](https://github.com/AOSC-Dev/aoscdk-rs/commit/b555dc87757a51a2e3d8e7155dd9910fadd50f2e))
    - Better ux logic ([`4e3d623`](https://github.com/AOSC-Dev/aoscdk-rs/commit/4e3d623a3643d7764f03b522e55d3a1bdaab3ba6))
    - Add set utc/rtc option ([`cabda31`](https://github.com/AOSC-Dev/aoscdk-rs/commit/cabda31fd4848fe2d466e7ef74f829a6338c4895))
    - Remove /etc/localtime after symlink timezone ([`30ec9bf`](https://github.com/AOSC-Dev/aoscdk-rs/commit/30ec9bfbb8215c7de7bc3477ba8f480b15c0f976))
    - Better set timezone ux ([`c2a3f65`](https://github.com/AOSC-Dev/aoscdk-rs/commit/c2a3f65e82b6fa4c57644d632cb355f3d4cd2c61))
    - Add zone select ([`67dad60`](https://github.com/AOSC-Dev/aoscdk-rs/commit/67dad60ab26498b0a9214f31f0814fb4bfec8113))
    - Add set_hwclock_tc() function ([`e82d638`](https://github.com/AOSC-Dev/aoscdk-rs/commit/e82d6388b424c929beec6d47981682e8e4fba7a3))
    - Add zone1970.tab parser ([`618fb1c`](https://github.com/AOSC-Dev/aoscdk-rs/commit/618fb1cc8a25e5c843d229b07649ddc0b78b62df))
    - Should not return an error when remove_file() fails, since it is already the last step ([`2e3d93f`](https://github.com/AOSC-Dev/aoscdk-rs/commit/2e3d93fde08418e59a7403ed2fc9eb7707beaf4c))
    - Add more err check ([`558eacc`](https://github.com/AOSC-Dev/aoscdk-rs/commit/558eacca25e5f7084bd4909711c3b0e780637280))
    - Add send_error marco to lint code ([`7ec2324`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7ec2324028f6b2e14643ef9392d5a6e0c2cdc4a9))
    - Add some error check ([`b0ab3d4`](https://github.com/AOSC-Dev/aoscdk-rs/commit/b0ab3d4f3abb1ae9f130da9c495d18f2a6df4beb))
    - Fallocate before download ([`530f69a`](https://github.com/AOSC-Dev/aoscdk-rs/commit/530f69a88268c70195412e2ab72344c85382b09a))
    - Add write_all error handle ([`ef4ccfe`](https://github.com/AOSC-Dev/aoscdk-rs/commit/ef4ccfe22be5648766340cc10713afd2bc004c13))
    - Use threads to download tarball and checksum ([`a945c8a`](https://github.com/AOSC-Dev/aoscdk-rs/commit/a945c8a5d1b88a34da3ff3df44c7ed35bd8acc3b))
    - Add download tarball error handle more message ([`d6272c5`](https://github.com/AOSC-Dev/aoscdk-rs/commit/d6272c5d2e738526b52a140f95179e877ee6ff58))
    - Add sha256sum match ([`f95ae86`](https://github.com/AOSC-Dev/aoscdk-rs/commit/f95ae86380fd22911a4bd1ec4b2f4b42223cf460))
    - No save of partition configuration ([`8b86b95`](https://github.com/AOSC-Dev/aoscdk-rs/commit/8b86b9559a3aa90eb77344c7b733e61bee154620))
    - Save user config to next time run deploykit read config ([`a063635`](https://github.com/AOSC-Dev/aoscdk-rs/commit/a063635b9b8e33ec1e92077bd5054b4a12405540))
    - If failed to download tarball, return error ([`bc1bf0d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/bc1bf0d168e9c7e186108da0c3defeee993d278a))
    - Update dependencies ([`ab7b14a`](https://github.com/AOSC-Dev/aoscdk-rs/commit/ab7b14a1a9955c9ebe1c572768f5828a8b2944d9))
    - Do not allow user install buildkit tarball ([`b9316a1`](https://github.com/AOSC-Dev/aoscdk-rs/commit/b9316a1f2e21fa47e142f2a3f3c79e134cf81dd0))
    - Make executing gparted non-blocking ([`2dbe653`](https://github.com/AOSC-Dev/aoscdk-rs/commit/2dbe6531d3c76659ad3de48227b3a28d0247b38a))
    - Add better support for retro mode ... ([`5531b4f`](https://github.com/AOSC-Dev/aoscdk-rs/commit/5531b4f18fab0c5995750f54e975f3b5364f674e))
    - (cargo-release) start next development iteration 0.2.1-alpha.0 ([`9183131`](https://github.com/AOSC-Dev/aoscdk-rs/commit/91831316098a7000c27990d4522f10cd3b6d6f69))
</details>

## v0.2.0 (2021-06-12)

<csr-id-8f998c20e04f2cf97c012c4a28c8942765aec4aa/>
<csr-id-b15b40d9192660f3bafd636c9204cbdecde89851/>
<csr-id-f4efbc3890451bd0286ced8ff20be2f3d4cbf86c/>
<csr-id-9c7470e8b64732695e3b1f49f49a0ea88aa51ffe/>
<csr-id-c73f089335d5eb7bb1fd8d595495ffb7f1e546dd/>
<csr-id-3c2562da0cc40a8070aa78d52beb4d975b1374a9/>
<csr-id-0c76b340bb8a2a3178524d56773c95df4e34040b/>
<csr-id-c514f1097043f87ed740c8eb2020625c3518952c/>
<csr-id-bffd73d0cb5c37eb4e3d66038f98f22fad7583d0/>
<csr-id-f69bba2ae81054a29c07ce3f38ef3e556d68b9ae/>
<csr-id-1a9ddf5efe40d7ee72d3ee86c90a7025d0f44480/>
<csr-id-e8dc68d4e81065369c45e99101b181cc08189deb/>
<csr-id-3e05bf9ba98d0cf519fabc61e8da411d91b09285/>
<csr-id-5997033950c139de294ac6f9d5546ba55a20df02/>
<csr-id-1a5b8cf5b35166b23a76a6ed96b336109d3e69d5/>
<csr-id-0fe7f6f7e19031a992b2f9cf2ad50ee39dc4690c/>
<csr-id-dd91c7bfa6c67f022b3d981960b408f70615e2a0/>
<csr-id-312477f4cddb6dbbe0c22b850aac0ab6715abe41/>
<csr-id-e10501bd06cbfdcf6461930980f6a9b0d0ad57d5/>
<csr-id-5c5fbf0020539d45b371b0a6e0c82e44f396e69b/>
<csr-id-e2f2ec92b8e5b693dd2b21d46b0293c3f7c95a1f/>
<csr-id-cd670c44cd8144a49556a932c778a0c5d1585fef/>
<csr-id-94f1195cdfc7a32011ecc12e1bfb3cb726c71c8e/>
<csr-id-eaf3330ed9aaa7705774e10e869750225d37ad9f/>
<csr-id-f34b0c10eeba509bf97ade2915d7e6da604aae27/>
<csr-id-80456c4a27f4ace14f1391dae4a721c9f643508a/>
<csr-id-c6100c10c7b7c23650bd523f048ce218a1c0e00d/>

### Other

 - <csr-id-8f998c20e04f2cf97c012c4a28c8942765aec4aa/> tweak UI strings
 - <csr-id-b15b40d9192660f3bafd636c9204cbdecde89851/> tui: fix progress indicator
 - <csr-id-f4efbc3890451bd0286ced8ff20be2f3d4cbf86c/> use update-initramfs to generate initrd
 - <csr-id-9c7470e8b64732695e3b1f49f49a0ea88aa51ffe/> refactor ...
   ... split install process into a separate module
 - <csr-id-c73f089335d5eb7bb1fd8d595495ffb7f1e546dd/> fix build
 - <csr-id-3c2562da0cc40a8070aa78d52beb4d975b1374a9/> format code and update dependencies
 - <csr-id-0c76b340bb8a2a3178524d56773c95df4e34040b/> migrate error handling to anyhow
 - <csr-id-c514f1097043f87ed740c8eb2020625c3518952c/> format code
 - <csr-id-bffd73d0cb5c37eb4e3d66038f98f22fad7583d0/> use tableview for variant list
 - <csr-id-f69bba2ae81054a29c07ce3f38ef3e556d68b9ae/> use the new AOSC OS image manifest system
 - <csr-id-1a9ddf5efe40d7ee72d3ee86c90a7025d0f44480/> fix some clippy warnings
 - <csr-id-e8dc68d4e81065369c45e99101b181cc08189deb/> add locale selection
 - <csr-id-3e05bf9ba98d0cf519fabc61e8da411d91b09285/> extract error handling to macro
 - <csr-id-5997033950c139de294ac6f9d5546ba55a20df02/> extract loading page to macros
 - <csr-id-1a5b8cf5b35166b23a76a6ed96b336109d3e69d5/> extract dialog creation function
 - <csr-id-0fe7f6f7e19031a992b2f9cf2ad50ee39dc4690c/> improve escape_chroot function
 - <csr-id-dd91c7bfa6c67f022b3d981960b408f70615e2a0/> preserve file permissions...
   ... when extracting tarballs
 - <csr-id-312477f4cddb6dbbe0c22b850aac0ab6715abe41/> configure user and hostname
 - <csr-id-e10501bd06cbfdcf6461930980f6a9b0d0ad57d5/> add finish page
 - <csr-id-5c5fbf0020539d45b371b0a6e0c82e44f396e69b/> fix quoting in grub install
 - <csr-id-e2f2ec92b8e5b693dd2b21d46b0293c3f7c95a1f/> show error message for grub install
 - <csr-id-cd670c44cd8144a49556a932c778a0c5d1585fef/> fix chroot escaping
 - <csr-id-94f1195cdfc7a32011ecc12e1bfb3cb726c71c8e/> make the folder before mounting efi partition
 - <csr-id-eaf3330ed9aaa7705774e10e869750225d37ad9f/> fix fat16/32 mounting
 - <csr-id-f34b0c10eeba509bf97ade2915d7e6da604aae27/> implement grub installation
 - <csr-id-80456c4a27f4ace14f1391dae4a721c9f643508a/> replace hardcoded path with actual impl
 - <csr-id-c6100c10c7b7c23650bd523f048ce218a1c0e00d/> config page done

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 31 commits contributed to the release over the course of 344 calendar days.
 - 27 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 ([`f6bd56d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/f6bd56da55c79af2e605fe94a52f59d675ef98a0))
    - Tweak UI strings ([`8f998c2`](https://github.com/AOSC-Dev/aoscdk-rs/commit/8f998c20e04f2cf97c012c4a28c8942765aec4aa))
    - Tui: fix progress indicator ([`b15b40d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/b15b40d9192660f3bafd636c9204cbdecde89851))
    - Use update-initramfs to generate initrd ([`f4efbc3`](https://github.com/AOSC-Dev/aoscdk-rs/commit/f4efbc3890451bd0286ced8ff20be2f3d4cbf86c))
    - Refactor ... ([`9c7470e`](https://github.com/AOSC-Dev/aoscdk-rs/commit/9c7470e8b64732695e3b1f49f49a0ea88aa51ffe))
    - Fix build ([`c73f089`](https://github.com/AOSC-Dev/aoscdk-rs/commit/c73f089335d5eb7bb1fd8d595495ffb7f1e546dd))
    - Format code and update dependencies ([`3c2562d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/3c2562da0cc40a8070aa78d52beb4d975b1374a9))
    - Migrate error handling to anyhow ([`0c76b34`](https://github.com/AOSC-Dev/aoscdk-rs/commit/0c76b340bb8a2a3178524d56773c95df4e34040b))
    - Format code ([`c514f10`](https://github.com/AOSC-Dev/aoscdk-rs/commit/c514f1097043f87ed740c8eb2020625c3518952c))
    - Use tableview for variant list ([`bffd73d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/bffd73d0cb5c37eb4e3d66038f98f22fad7583d0))
    - Use the new AOSC OS image manifest system ([`f69bba2`](https://github.com/AOSC-Dev/aoscdk-rs/commit/f69bba2ae81054a29c07ce3f38ef3e556d68b9ae))
    - Fix some clippy warnings ([`1a9ddf5`](https://github.com/AOSC-Dev/aoscdk-rs/commit/1a9ddf5efe40d7ee72d3ee86c90a7025d0f44480))
    - Add locale selection ([`e8dc68d`](https://github.com/AOSC-Dev/aoscdk-rs/commit/e8dc68d4e81065369c45e99101b181cc08189deb))
    - Extract error handling to macro ([`3e05bf9`](https://github.com/AOSC-Dev/aoscdk-rs/commit/3e05bf9ba98d0cf519fabc61e8da411d91b09285))
    - Extract loading page to macros ([`5997033`](https://github.com/AOSC-Dev/aoscdk-rs/commit/5997033950c139de294ac6f9d5546ba55a20df02))
    - Extract dialog creation function ([`1a5b8cf`](https://github.com/AOSC-Dev/aoscdk-rs/commit/1a5b8cf5b35166b23a76a6ed96b336109d3e69d5))
    - Multiple changes... ([`7962c8f`](https://github.com/AOSC-Dev/aoscdk-rs/commit/7962c8fbb417acfb25df73270c3311e2e12085ea))
    - Improve escape_chroot function ([`0fe7f6f`](https://github.com/AOSC-Dev/aoscdk-rs/commit/0fe7f6f7e19031a992b2f9cf2ad50ee39dc4690c))
    - Preserve file permissions... ([`dd91c7b`](https://github.com/AOSC-Dev/aoscdk-rs/commit/dd91c7bfa6c67f022b3d981960b408f70615e2a0))
    - Configure user and hostname ([`312477f`](https://github.com/AOSC-Dev/aoscdk-rs/commit/312477f4cddb6dbbe0c22b850aac0ab6715abe41))
    - Add finish page ([`e10501b`](https://github.com/AOSC-Dev/aoscdk-rs/commit/e10501bd06cbfdcf6461930980f6a9b0d0ad57d5))
    - Fix quoting in grub install ([`5c5fbf0`](https://github.com/AOSC-Dev/aoscdk-rs/commit/5c5fbf0020539d45b371b0a6e0c82e44f396e69b))
    - Show error message for grub install ([`e2f2ec9`](https://github.com/AOSC-Dev/aoscdk-rs/commit/e2f2ec92b8e5b693dd2b21d46b0293c3f7c95a1f))
    - Fix chroot escaping ([`cd670c4`](https://github.com/AOSC-Dev/aoscdk-rs/commit/cd670c44cd8144a49556a932c778a0c5d1585fef))
    - Make the folder before mounting efi partition ([`94f1195`](https://github.com/AOSC-Dev/aoscdk-rs/commit/94f1195cdfc7a32011ecc12e1bfb3cb726c71c8e))
    - Fix fat16/32 mounting ([`eaf3330`](https://github.com/AOSC-Dev/aoscdk-rs/commit/eaf3330ed9aaa7705774e10e869750225d37ad9f))
    - Implement grub installation ([`f34b0c1`](https://github.com/AOSC-Dev/aoscdk-rs/commit/f34b0c10eeba509bf97ade2915d7e6da604aae27))
    - Replace hardcoded path with actual impl ([`80456c4`](https://github.com/AOSC-Dev/aoscdk-rs/commit/80456c4a27f4ace14f1391dae4a721c9f643508a))
    - Partial installation impl ([`0665dec`](https://github.com/AOSC-Dev/aoscdk-rs/commit/0665dec5da30f04dd99c3cb14b04f3fa18bc6f73))
    - Config page done ([`c6100c1`](https://github.com/AOSC-Dev/aoscdk-rs/commit/c6100c10c7b7c23650bd523f048ce218a1c0e00d))
    - Initial commit ([`d94ab8e`](https://github.com/AOSC-Dev/aoscdk-rs/commit/d94ab8e2eb2c8185335d3a737190493418ad25d0))
</details>

