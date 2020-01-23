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

//! Search, iterate and modify data.

use crate::error::Result;
use crate::Session;
use std::marker::PhantomData;
use wiredtiger_sys::WT_CURSOR;

pub struct Cursor<'a> {
    inner: Option<*mut WT_CURSOR>,
    session: PhantomData<&'a Session<'a>>,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new_unchecked(cursor: *mut WT_CURSOR) -> Self {
        debug_assert!(!cursor.is_null());
        Self {
            inner: Some(cursor),
            session: PhantomData,
        }
    }

    pub fn close(&mut self) -> Result<()> {
        if let Some(cursor) = self.inner {
            debug_assert!(!cursor.is_null());
            unsafe {
                let close = (*cursor).close.unwrap();
                wt_try!(close(cursor));
            }

            self.inner.take();
        }
        Ok(())
    }
}

impl<'a> Drop for Cursor<'a> {
    fn drop(&mut self) {
        let result = self.close();
        match result {
            Ok(_) => (),
            Err(error) => eprintln!("error happened when auto close cursor: {}", error),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_utils, Connection};

    #[test]
    fn test_open_cursor() {
        let home = "target/wt_open_cursor";
        test_utils::ensure_wt_home(home, false);
        let conn = Connection::open(home, "create").unwrap();
        let session = conn.open_session("").unwrap();
        session
            .create("table:test_table", "key_format=S,value_format=S")
            .unwrap();
        let mut cursor = session.open_cursor("table:test_table", "").unwrap();
        cursor.close().unwrap();
        session.drop("table:test_table", "").unwrap();
    }
}
