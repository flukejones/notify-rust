# Changelog

## [Unreleased](https://github.com/hoodie/notify-rust/compare/v4.5.10...HEAD) (2022-12-03)

* Update to zbus 4.0.1. Remove dbus junk

### Features

* update zbus dependency to 3.x
  ([4c496dc](https://github.com/hoodie/notify-rust/commit/4c496dc8c718b38f2108c9858f1e0c2e15cd5bff))

### [v4.5.10](https://github.com/hoodie/notify-rust/compare/v4.5.9...v4.5.10) (2022-09-25)

#### Fixes

* lower minimum rustc version to 1.59.0 for linux (dbus)
  ([c0ffeed](https://github.com/hoodie/notify-rust/commit/c0ffeede93816f601a0a8701fb3510c052ce3a0e))

### [v4.5.9](https://github.com/hoodie/notify-rust/compare/v4.5.8...v4.5.9) (2022-09-21)

#### Fixes

* raise minimum rustc version to 1.60.0 for zbus
  ([b3b9aba](https://github.com/hoodie/notify-rust/commit/b3b9aba88fbacfd2c2c406e58cbc020c46895879))
* update winrt-notification to a maintained fork, closes #148
  ([7cc6824](https://github.com/hoodie/notify-rust/commit/7cc682428d2f5f0c2335c8ff141fb71af1e01a26))

### [v4.5.8](https://github.com/hoodie/notify-rust/compare/v4.5.7...v4.5.8) (2022-03-26)

#### Fixes

* re-exported types from mac-notification-sys
  ([80daef6](https://github.com/hoodie/notify-rust/commit/80daef6b0dfda0d61697d118c824930bb00a8a35))

### [v4.5.7](https://github.com/hoodie/notify-rust/compare/v4.5.6...v4.5.7) (2022-03-20)

#### Fixes

* **deps:** update rust crate mac-notification-sys to 0.5.0
  ([1f3a9f5](https://github.com/hoodie/notify-rust/commit/1f3a9f5c36c17775885fd6adb80bbf0295219e45))

### [v4.5.6](https://github.com/hoodie/notify-rust/compare/v4.5.5...v4.5.6) (2022-02-04)

#### Fixes

* update crates zbus to v2, zvariant to v3, zvariant_derive to v3
  ([14bca58](https://github.com/hoodie/notify-rust/commit/14bca588efbc88d6a1e00b5abcb5be7db44abfcf))
* **deps:** update rust crate zbus to v2
  ([04901a8](https://github.com/hoodie/notify-rust/commit/04901a85fd587ed7482a0bf222d76d7c898369ef))
* **deps:** update rust crate image to 0.24
  ([c7fa276](https://github.com/hoodie/notify-rust/commit/c7fa276cab7ffdd2b6e825334837a3515599a3a9))

### [v4.5.5](https://github.com/hoodie/notify-rust/compare/v4.5.4...v4.5.5) (2021-11-04)

#### Fixes

* **deps:** update rust crate winrt-notification to 0.5
  ([6620110](https://github.com/hoodie/notify-rust/commit/6620110a5a8e37b4ac738ccd3df87cda88f57ce8))

### [v4.5.4](https://github.com/hoodie/notify-rust/compare/v4.5.3...v4.5.4) (2021-10-08)

#### Fixes

* update winrt-notification to 0.4
  ([c94e111](https://github.com/hoodie/notify-rust/commit/c94e111f320c75015de9d2177d12f4dce218804b))

### [v4.5.3](https://github.com/hoodie/notify-rust/compare/v4.5.2...v4.5.3) (2021-09-16)

#### Fixes

* clear up documentation and flip env switch for dual stack
  ([3a3b175](https://github.com/hoodie/notify-rust/commit/3a3b1750ce24d2ba54437cfe65a9c8d7dd251cf2))

### [v4.5.2](https://github.com/hoodie/notify-rust/compare/v4.5.1...v4.5.2) (2021-05-14)

#### Fixes

* clear up documentation of action handling
  ([ad35d34](https://github.com/hoodie/notify-rust/commit/ad35d34f93bed6c4f926403d6495972f9dcba273))

### [v4.5.1](https://github.com/hoodie/notify-rust/compare/v4.5.0...v4.5.1) (2021-05-13)

#### Fixes

* export of ActionResponse
  ([3bf2f37](https://github.com/hoodie/notify-rust/commit/3bf2f37836902334f556d66c609d5d52d732284a))

## [v4.5.0](https://github.com/hoodie/notify-rust/compare/v4.4.1...v4.5.0) (2021-05-05)

### Features

* **NotificationHandle:** add close reason handling
  ([01f9980](https://github.com/hoodie/notify-rust/commit/01f9980e20a687f37391b8c32675436167b4f435)),
  closes [#104](https://github.com/hoodie/notify-rust/issues/104)

### [v4.4.1](https://github.com/hoodie/notify-rust/compare/v4.4.0...v4.4.1) (2021-05-01)

#### Fixes

* notify with __closed when notification is closed in zbus implementation
  ([3a9c206](https://github.com/hoodie/notify-rust/commit/3a9c2066929bace67bf88b5882f4296906a7a7dc))

## [v4.4.0](https://github.com/hoodie/notify-rust/compare/v4.3.0...v4.4.0) (2021-04-25)

### Features

* add schedule_raw() for f64 timestamps
  ([b8f811b](https://github.com/hoodie/notify-rust/commit/b8f811b030c94224a8b6c17cdd49f09a41a417a4))
* add schedule method to Notification
  ([30f1741](https://github.com/hoodie/notify-rust/commit/30f1741ca8818fd4552de0b53bf7fc19511886bc))
* create macos schedule_notification method
  ([72bda94](https://github.com/hoodie/notify-rust/commit/72bda94d3e0281d8c0dc88343f643099d1963520))

## [v4.3.0](https://github.com/hoodie/notify-rust/compare/v4.2.2...v4.3.0) (2021-02-27)

### Features

* Convert DynamicImage::ImageRgba8
  ([87e92b5](https://github.com/hoodie/notify-rust/commit/87e92b5d717edcd75f94f66b0f7aed1f04363459))
* Implement TryFrom trait for RgbImage and RgbaImage
  ([69c2b1e](https://github.com/hoodie/notify-rust/commit/69c2b1e2417399ea08a13463218c1ef34f64ab25))
* Implement converting image with alpha
  ([d25ab47](https://github.com/hoodie/notify-rust/commit/d25ab47b4bdc0d7cab2802787e3f086e32dad29d))
* make zbus backend the default
  ([582b87e](https://github.com/hoodie/notify-rust/commit/582b87ec85647e66058f1857bc0a169411ebeb3b))

### [v4.2.2](https://github.com/hoodie/notify-rust/compare/v4.2.1...v4.2.2) (2021-01-08)

#### Fixes

* remove another stray println
  ([bd6ab59](https://github.com/hoodie/notify-rust/commit/bd6ab59d04d23f25da1f333677fc89d90ff2aaf3))

### [v4.2.1](https://github.com/hoodie/notify-rust/compare/v4.2.0...v4.2.1) (2021-01-08)

#### Fixes

* **deps:** update zbus
  ([684d031](https://github.com/hoodie/notify-rust/commit/684d03161470ad119095c86999f169a352d2e4d3))
* **deps:** update rust crate dbus to 0.9
  ([96f84f4](https://github.com/hoodie/notify-rust/commit/96f84f4e1c2d025d0d42488f07c178fba012a030))

## [v4.2.0](https://github.com/hoodie/notify-rust/compare/v4.1.1...v4.2.0) (2021-01-08)

### Features

* make custom hints unique
  ([f6ec445](https://github.com/hoodie/notify-rust/commit/f6ec4453367db915fb1523c8ea223ed3e9cae800)),
  closes [#88](https://github.com/hoodie/notify-rust/issues/88)

### Fixes

* remove stray dbug!()
  ([b67c1d5](https://github.com/hoodie/notify-rust/commit/b67c1d504df2c99f24a8ceef0079b4764ebc52d7))

### [v4.1.1](https://github.com/hoodie/notify-rust/compare/v4.1.0...v4.1.1) (2021-01-07)

#### Fixes

* remove stray println
  ([15b3ecd](https://github.com/hoodie/notify-rust/commit/15b3ecd2be6ec09a1a490187a24ed40e03526d77))

## [v4.1.0](https://github.com/hoodie/notify-rust/compare/v4.0.0...v4.1.0) (2021-01-06)

### Features

* add zbus version
  ([58d38ba](https://github.com/hoodie/notify-rust/commit/58d38ba98e19b6f382f577416b039bd0dcb4491c))

### Fixes

* **deps:** update rust crate image to 0.23
  ([1dd236d](https://github.com/hoodie/notify-rust/commit/1dd236d95b9bd47ea4cbfd75d648ba6e93d8a9a0))

## [v4.0.0](https://github.com/hoodie/notify-rust/compare/v3.6.3...v4.0.0) (2020-06-06)

### ⚠ BREAKING CHANGE

* remove `From<&str>`* restructure modules and exports

### Features

* **windows:** additions to the API (#69)
  ([1d9cb0e](https://github.com/hoodie/notify-rust/commit/1d9cb0eb73fdf7f442757a63f2131cb42c4c150f))
* make notification non-exhaustive
  ([0304274](https://github.com/hoodie/notify-rust/commit/03042744a05fdab5eff1eee5023c93b2930710d2))
* make error non-exhaustive
  ([26f96e4](https://github.com/hoodie/notify-rust/commit/26f96e4a90916496b26a0c955a878b0fe83b9aab))
* drop redundant name prefixes
  ([faf3123](https://github.com/hoodie/notify-rust/commit/faf3123bb375a58b57930727da9a09df49830c3e))
* restructure modules and exports
  ([45be84c](https://github.com/hoodie/notify-rust/commit/45be84ce976ac3a18d674df7a412ec638861e6b0))
* .image() no longer silently fails
  ([8b215bd](https://github.com/hoodie/notify-rust/commit/8b215bd8dfd31655711c522553bb130552930201))

### Fixes

* reexport NotificationHandle
  ([00edbc9](https://github.com/hoodie/notify-rust/commit/00edbc9a2e44db642d02abfcb6931f0e3bb77563))

### [v3.6.3](https://github.com/hoodie/notify-rust/compare/v3.6.2...v3.6.3) (2019-11-02)

#### Fixes

* **deps:** update rust crate lazy_static to 1.4
  ([687e34d](https://github.com/hoodie/notify-rust/commit/687e34d7a73db36eb2bdeb20e27c8ed18b08f4c1))
* build again after merge conflict
  ([bcfc8c8](https://github.com/hoodie/notify-rust/commit/bcfc8c8d41d38bcc51a78ca5febcb4f1adb4d810))
* test-build without `--features image` 🙄
  ([3eead0b](https://github.com/hoodie/notify-rust/commit/3eead0b30d189315828a7e4835526e551f0e81a0))
* test-build with `--features image`
  ([60e963d](https://github.com/hoodie/notify-rust/commit/60e963de1ea40e29ab09599d26160b206b8acf31))
* test-build with `--features image`
  ([92217a2](https://github.com/hoodie/notify-rust/commit/92217a2c83b4c083076978061059175ecb9048ee))

### [v3.6.2](https://github.com/hoodie/notify-rust/compare/v3.6.1...v3.6.2) (2019-08-11)

#### Fixes

* test-build without `--features image` 🙄
  ([0524a5f](https://github.com/hoodie/notify-rust/commit/0524a5fba819a7470525cfdcbcf4dcb4db4b3623))

### [v3.6.1](https://github.com/hoodie/notify-rust/compare/v3.6.0...v3.6.1) (2019-08-11)

#### Fixes

* test-build with `--features image`
  ([8ee6998](https://github.com/hoodie/notify-rust/commit/8ee69984cb5350287581de2b453f225d1fef3de2))

## [v3.6.0](https://github.com/hoodie/notify-rust/compare/v3.5.0...v3.6.0) (2019-05-06)

## [v3.5.0](https://github.com/hoodie/notify-rust/compare/v3.4.3...v3.5.0) (2018-10-21)

### [v3.4.3](https://github.com/hoodie/notify-rust/compare/v3.4.0...v3.4.3) (2018-10-13)

## [v3.4.0](https://github.com/hoodie/notify-rust/compare/v3.2.1...v3.4.0) (2017-05-21)

### [v3.2.1](https://github.com/hoodie/notify-rust/compare/v3.1.1...v3.2.1) (2016-09-07)

### [v3.1.1](https://github.com/hoodie/notify-rust/compare/v3.1.0...v3.1.1) (2016-03-03)

## [v3.1.0](https://github.com/hoodie/notify-rust/compare/v3.0.4...v3.1.0) (2016-03-01)

### [v3.0.4](https://github.com/hoodie/notify-rust/compare/v3.0.3...v3.0.4) (2016-02-15)

### [v3.0.3](https://github.com/hoodie/notify-rust/compare/v3.0.2...v3.0.3) (2016-02-15)

### [v3.0.2](https://github.com/hoodie/notify-rust/compare/v3.0.1...v3.0.2) (2016-02-15)

### [v3.0.1](https://github.com/hoodie/notify-rust/compare/v3.0.0...v3.0.1) (2015-10-23)

## [v3.0.0](https://github.com/hoodie/notify-rust/compare/v2.1.0...v3.0.0) (2015-10-01)

## [v2.1.0](https://github.com/hoodie/notify-rust/compare/v2.0.0...v2.1.0) (2015-09-27)

## [v2.0.0](https://github.com/hoodie/notify-rust/compare/v1.1.0...v2.0.0) (2015-08-04)

## [v1.1.0](https://github.com/hoodie/notify-rust/compare/v1.0.1...v1.1.0) (2015-08-03)

### [v1.0.1](https://github.com/hoodie/notify-rust/compare/v1.0.0...v1.0.1) (2015-07-19)

## [v1.0.0](https://github.com/hoodie/notify-rust/compare/v0.9.0...v1.0.0) (2015-07-12)

## [v0.9.0](https://github.com/hoodie/notify-rust/compare/v0.8.0...v0.9.0) (2015-06-30)

## [v0.8.0](https://github.com/hoodie/notify-rust/compare/v0.0.8...v0.8.0) (2015-06-19)

### [v0.0.8](https://github.com/hoodie/notify-rust/compare/v0.0.7...v0.0.8) (2015-06-19)

### [v0.0.7](https://github.com/hoodie/notify-rust/compare/v0.0.6...v0.0.7) (2015-06-13)

### [v0.0.6](https://github.com/hoodie/notify-rust/compare/v0.0.4...v0.0.6) (2015-06-08)

### [v0.0.4](https://github.com/hoodie/notify-rust/compare/v0.0.3...v0.0.4) (2015-05-30)

### [v0.0.3](https://github.com/hoodie/notify-rust/compare/v0.0.2...v0.0.3) (2015-05-24)

### v0.0.2 (2015-05-22)
