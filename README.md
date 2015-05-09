# Drive 


|Crate|Travis|Appveyor|Coverage|
|:-------:|:-------:|:------:|:------:|
|[![](http://meritbadge.herokuapp.com/drive)](https://crates.io/crates/drive)|[![Build Status](https://travis-ci.org/maidsafe/drive.svg?branch=master)](https://travis-ci.org/maidsafe/drive)|[![Build status](https://ci.appveyor.com/api/projects/status/k5mtki2neaoa8voq/branch/master?svg=true)](https://ci.appveyor.com/project/maidsafe/drive/branch/master)|[![Coverage Status](https://coveralls.io/repos/maidsafe/drive/badge.svg)](https://coveralls.io/r/dirvine/drive)|

|[API Documentation](http://maidsafe.github.io/drive/)| [MaidSafe System Documention](http://systemdocs.maidsafe.net/) | [MaidSafe web site](http://www.maidsafe.net) | [Safe Community site](https://forum.safenetwork.io) |


#Overview


A cross platform [virtual file-system in userspace](http://en.wikipedia.org/wiki/Filesystem_in_Userspace) (drive) that will appear as a regular drive on the operating system. The interface is a POSIX-like API and this is exposed in every OS. May include a webdav interface where possible. 

IOS and Android…etc… may require a driverless option, further consideration will also be required (webdav ?) to provide the same cross platform/OS compatibility. 

This drive can provide a blocking call to be used as a stand alone application, or a threaded call to enable a drive to be mounted from any application. 

#Build Prerequisites

##Linux

Requires fuse dev files in ubuntu `sudo apt-get install libfuse-dev`

##OSX

Requires osxfuse (easiest method is to use Homebrew and `brew install osxfuse`

##Bsd 

Likely working [Puffs](http://www.netbsd.org/docs/puffs/) and will require fuse-development library installed, but requires tests and CI integration.

##Windows

Currently unimplemented and will require the [windows driver frameworks](https://github.com/Microsoft/Windows-Driver-Frameworks) as  per these [examples](https://github.com/Microsoft/windows-driver-samples) 


#Todo
- [ ] Finalise API
- [ ] Confirm Bsd 
- [ ] Provide simple example (mirror) 
- [ ] API version 0.0.8
- [ ] Add windows driver version
- [ ] API version 0.0.9
- [ ] Webdav integration
- [ ] API version 0.1.0
