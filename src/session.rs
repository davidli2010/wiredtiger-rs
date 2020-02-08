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

//! A context for performing database operations.

use crate::error::Result;
use crate::{Connection, Cursor};
use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr;
use wiredtiger_sys::{WT_CURSOR, WT_SESSION};

macro_rules! session_api {
    ($session: ident, $api: ident) => {
        unsafe {
            let session = $session.inner.expect("session is null");
            let api = (*session).$api.expect("null function pointer");
            (session, api)
        }
    };
}

pub struct Session<'a> {
    inner: Option<*mut WT_SESSION>,
    conn: PhantomData<&'a Connection>,
}

impl<'a> Session<'a> {
    pub(crate) fn new_unchecked(session: *mut WT_SESSION) -> Self {
        debug_assert!(!session.is_null());
        Self {
            inner: Some(session),
            conn: PhantomData,
        }
    }

    pub fn close<C: AsRef<str>>(&mut self, config: C) -> Result<()> {
        if let Some(session) = self.inner {
            debug_assert!(!session.is_null());
            let c_config = CString::new(config.as_ref().as_bytes()).unwrap();

            unsafe {
                let close = (*session).close.unwrap();
                wt_try!(close(session, c_config.as_ptr()));
            }

            self.inner.take();
        }
        Ok(())
    }

    pub fn create<N: AsRef<str>, C: AsRef<str>>(&self, name: N, config: C) -> Result<()> {
        let c_name = CString::new(name.as_ref().as_bytes()).unwrap();
        let c_config = CString::new(config.as_ref().as_bytes()).unwrap();

        let (session, create) = session_api!(self, create);
        unsafe {
            wt_try!(create(session, c_name.as_ptr(), c_config.as_ptr()));
        }
        Ok(())
    }

    pub fn drop<N: AsRef<str>, C: AsRef<str>>(&self, name: N, config: C) -> Result<()> {
        let c_name = CString::new(name.as_ref().as_bytes()).unwrap();
        let c_config = CString::new(config.as_ref().as_bytes()).unwrap();

        let (session, drop) = session_api!(self, drop);
        unsafe {
            wt_try!(drop(session, c_name.as_ptr(), c_config.as_ptr()));
        }
        Ok(())
    }

    pub fn open_cursor<U: AsRef<str>, C: AsRef<str>>(&self, uri: U, config: C) -> Result<Cursor> {
        let (session, open_cursor) = session_api!(self, open_cursor);

        let c_uri = CString::new(uri.as_ref().as_bytes()).unwrap();
        let c_config = CString::new(config.as_ref().as_bytes()).unwrap();

        let mut cursor: *mut WT_CURSOR = ptr::null_mut();
        unsafe {
            wt_try!(open_cursor(
                session,
                c_uri.as_ptr(),
                ptr::null_mut(),
                c_config.as_ptr(),
                &mut cursor as *mut *mut WT_CURSOR
            ));
            assert!(!cursor.is_null());
            Ok(Cursor::new_unchecked(cursor))
        }
    }
}

impl<'a> Drop for Session<'a> {
    fn drop(&mut self) {
        let result = self.close("");
        match result {
            Ok(_) => (),
            Err(error) => eprintln!("error happened when auto close session: {}", error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils;

    #[test]
    fn test_open_session() {
        let home = "target/wt_open_session";
        test_utils::make_work_dir(home, false);
        let conn = Connection::open(home, "create").unwrap();
        let mut session = conn.open_session("").unwrap();
        session.close("").unwrap();
    }

    #[test]
    fn test_create_table() {
        let home = "target/wt_create_table";
        test_utils::make_work_dir(home, false);
        let conn = Connection::open(home, "create").unwrap();
        let session = conn.open_session("").unwrap();
        session
            .create("table:test_table", "key_format=S,value_format=S")
            .unwrap();
        session.drop("table:test_table", "").unwrap();
    }
}
