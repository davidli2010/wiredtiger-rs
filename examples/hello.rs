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

//! Example of how to create and open a database.

use test_utils;
use wiredtiger::*;

fn main() {
    let home = test_utils::example_setup();

    // Open a connection to the database, creating it if necessary.
    let conn = Connection::open(home, "create").unwrap();

    // Open a session for the current thread's work.
    let _session = conn.open_session("").unwrap();

    // Do some work...

    // Note: session and conn are implicitly closed when go out of this scope.
}
