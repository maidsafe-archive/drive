// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.                                                                 */

#![crate_name = "drive"]
#![crate_type = "lib"]
#![forbid(bad_style, missing_docs, warnings)]
#![deny(deprecated, drop_with_repr_extern, improper_ctypes, non_shorthand_field_patterns,
        overflowing_literals, plugin_as_library, private_no_mangle_fns, private_no_mangle_statics,
        raw_pointer_derive, stable_features, unconditional_recursion, unknown_lints, unsafe_code,
        unused, unused_allocation, unused_attributes, unused_comparisons, unused_features,
        unused_parens, while_true)]
#![warn(trivial_casts, trivial_numeric_casts, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, variant_size_differences)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/maidsafe/QA/master/Images/maidsafe_logo.png",
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
