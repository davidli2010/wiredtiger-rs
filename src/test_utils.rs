// Copyright 2020 David Li
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Test Utilities.

use std::path::Path;

/// Ensure the WiredTiger's home is existed.
///
/// `home` is the directory of WiredTiger, we'll create it if it's not existed.
/// `clear` indicates whether to clean up the `home`.
pub fn ensure_wt_home<T: AsRef<Path>>(home: T, clear: bool) {
    let home = home.as_ref();
    if home.exists() {
        if clear {
            std::fs::remove_dir_all(home).unwrap();
            std::fs::create_dir(home).unwrap();
        }
    } else {
        std::fs::create_dir(home).unwrap();
    }
}
