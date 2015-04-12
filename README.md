# drive 



|Travis build and test status|Appveyor build and test status (Windows)|Code Coverage|
|:--------------------------:|:--------------------------------------:|:-----------:|
[![Build Status](https://travis-ci.org/dirvine/drive.svg?branch=master)](https://travis-ci.org/dirvine/drive)|[![Build status](https://ci.appveyor.com/api/projects/status/jsuo65sa631h0kav?svg=true)](https://ci.appveyor.com/project/dirvine/drive)|[![Coverage Status](https://coveralls.io/repos/dirvine/drive/badge.svg)](https://coveralls.io/r/dirvine/drive)|

|[API Documentation](http://dirvine.github.io/drive/)| [MaidSafe System Documention](http://systemdocs.maidsafe.net/) | [MaidSafe web site](http:://www.maidsafe.net) | [Safe Community site](http:://www.maidsafe.org) |

#Overview

A cross platform [virtual file-system in userspace](http://en.wikipedia.org/wiki/Filesystem_in_Userspace) (drive) that will appear as a regular drive an the operating system. The interface is a POSIX like API and this is exposed in every OS. May include a webdav interface where possible. 

IOS and Android etc. may require a driverless option and will require further consideration (webdav ?) to provde the same cross platform/OS compatibility. 

This drive can provide a blocking call to be used as a stand alone application or a threaded call to all a drive to be mounted from any application. 

#Build Prerequisites

##Linux

Requires fuse dev files. in ubuntu `sudo apt-get install libfuse-dev`

##OSX

Requires osxfuse (easiest method is to use homebrew and `brew install osxfuse`

##Bsd 

Likely workign [Puffs](http://www.netbsd.org/docs/puffs/) and will require fuse-development library installed, but requires tests and CI integration.

##Windows

Currently unimplemented and will require the [windows driver frameworks](https://github.com/Microsoft/Windows-Driver-Frameworks) as  per these [examples](https://github.com/Microsoft/windows-driver-samples) 


#Todo
- [ ] Finalise API
- [ ] Confirm Bsd 
- [ ] Povide simple example (mirror) 
- [ ] API version 0.0.8
- [ ] Add windows driver version
- [ ] API version 0.0.9
- [ ] Webdav integration
- [ ] API version 0.1.0
