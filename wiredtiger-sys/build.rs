// Copyright 2020 David Li <davidli2010@foxmail.com>
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

use bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    let wt_home =
        env::var("WIREDTIGER_HOME").expect("WIREDTIGER_HOME environment variable is not defined");
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR environment variable is not defined");

    println!("cargo:rustc-link-search=native={}/lib", wt_home);
    println!("cargo:rustc-link-lib=static=wiredtiger");
    println!("cargo:rerun-if-changed=wrapper.h");

    bindgen::Builder::default()
        .clang_arg(format!("-I{}/include", wt_home))
        .header("wrapper.h")
        .whitelist_var("WIREDTIGER.*")
        .whitelist_var("WT.*")
        .whitelist_type("wt.*")
        .whitelist_type("WT.*")
        .whitelist_function("wiredtiger.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate wiredtiger's bindings")
        .write_to_file(PathBuf::from(out_dir).join("bindings.rs"))
        .expect("Unable to write wiredtiger's bindings");
}
