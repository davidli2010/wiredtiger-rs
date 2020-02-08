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

use std::path::{Path, PathBuf};

/// Delete the existing work directory, then create a new one.
///
/// `clear` indicates whether to clean up the `dir` if it's existing.
pub fn make_work_dir<T: AsRef<Path>>(dir: T, clear: bool) {
    let dir = dir.as_ref();
    if dir.exists() {
        if clear {
            clean_work_dir(dir);
            std::fs::create_dir(dir).unwrap();
        }
    } else {
        std::fs::create_dir(dir).unwrap();
    }
}

/// Remove the work directory.
pub fn clean_work_dir<T: AsRef<Path>>(dir: T) {
    let dir = dir.as_ref();
    if dir.exists() {
        std::fs::remove_dir_all(dir).unwrap();
    }
}

/// Create a home directory for the example programs.
pub fn example_setup() -> PathBuf {
    let home = std::env::var("WT_HOME").expect("WT_HOME is not defined");
    if home.is_empty() {
        panic!("WT_HOME is empty");
    }
    make_work_dir(&home, true);
    PathBuf::from(home)
}
