// Copyright 2015 MaidSafe.net limited
//
// This Safe Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the Safe Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0, found in the root
// directory of this project at LICENSE, COPYING and CONTRIBUTOR respectively and also
// available at: http://maidsafe.net/network-platform-licensing
//
// Unless required by applicable law or agreed to in writing, the Safe Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS
// OF ANY KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations relating to
// use of the Safe Network Software.                                                                */

#![crate_name = "drive"]
#![crate_type = "lib"]
#![doc(html_logo_url = "http://maidsafe.net/img/Resources/branding/maidsafe_logo.fab2.png",
       html_favicon_url = "http://maidsafe.net/img/favicon.ico",
              html_root_url = "http://dirvine.github.io/dirvine/drive/")]
//!#Drive
//!A cross platform virtual file-system in userspace (drive) that will appear as a regular drive on the operating system.
//! The interface is a POSIX-like API and this is exposed in every OS. May include a webdav interface where possible.
//!IOS and Android etc. may require a driverless option and will require further consideration (webdav ?) to provide the
//! same cross platform/OS compatibility.
//!This drive can provide a blocking call to be used as a stand alone application or a threaded call to enable a drive to be mounted from any application.
#[test]
fn dummy()  {
}
