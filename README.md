# ***This repository is no longer maintained***
# It has been moved to the maidsafe-archive organisation for reference only
#
#
#
#
# Drive

**Maintainer:** Spandan Sharma (spandan.sharma@maidsafe.net)

|Crate|Linux/OS X|Windows|Coverage|Issues|
|:---:|:--------:|:-----:|:------:|:----:|
|[![](http://meritbadge.herokuapp.com/drive)](https://crates.io/crates/drive)|[![Build Status](https://travis-ci.org/maidsafe/drive.svg?branch=master)](https://travis-ci.org/maidsafe/drive)|[![Build status](https://ci.appveyor.com/api/projects/status/so3q2w6g8vey2avl/branch/master?svg=true)](https://ci.appveyor.com/project/MaidSafe-QA/drive/branch/master)|[![Coverage Status](https://coveralls.io/repos/maidsafe/drive/badge.svg)](https://coveralls.io/r/maidsafe/drive)|[![Stories in Ready](https://badge.waffle.io/maidsafe/drive.png?label=ready&title=Ready)](https://waffle.io/maidsafe/drive)|

| [API Documentation - master branch](http://docs.maidsafe.net/drive/master) | [MaidSafe website](http://maidsafe.net) | [SAFE Network Forum](https://forum.safenetwork.io) |
|:------:|:-------:|:-------:|:-------:|

## Overview

A cross platform [virtual file-system in userspace](http://en.wikipedia.org/wiki/Filesystem_in_Userspace) (drive) that will appear as a regular drive on the operating system. The interface is a POSIX-like API and this is exposed in every OS. May include a webdav interface where possible.

IOS and Android…etc… may require a driverless option, further consideration will also be required (webdav ?) to provide the same cross platform/OS compatibility.

This drive can provide a blocking call to be used as a stand alone application, or a threaded call to enable a drive to be mounted from any application.

## Prerequisites

### Linux

Requires fuse dev files in ubuntu `sudo apt-get install libfuse-dev`

### OS X

Requires osxfuse (easiest method is to use Homebrew and `brew install osxfuse`

### BSD

Likely working [Puffs](http://www.netbsd.org/docs/puffs/) and will require fuse-development library installed, but requires tests and CI integration.

### Windows

Currently unimplemented and will require the [windows driver frameworks](https://github.com/Microsoft/Windows-Driver-Frameworks) as per these [examples](https://github.com/Microsoft/windows-driver-samples)

## Todo Items
- [ ] Finalise API
- [ ] Confirm Bsd
- [ ] Provide simple example (mirror)
- [ ] API version 0.0.8
- [ ] Add windows driver version
- [ ] API version 0.0.9
- [ ] Webdav integration
- [ ] API version 0.1.0
- [ ] Investigate midipix and DokanY.

## License

Licensed under either of

* the MaidSafe.net Commercial License, version 1.0 or later ([LICENSE](LICENSE))
* the General Public License (GPL), version 3 ([COPYING](COPYING) or http://www.gnu.org/licenses/gpl-3.0.en.html)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the MaidSafe Contributor Agreement, version 1.1 ([CONTRIBUTOR]
(CONTRIBUTOR)), shall be dual licensed as above, and you agree to be bound by the terms of the
MaidSafe Contributor Agreement, version 1.1.
