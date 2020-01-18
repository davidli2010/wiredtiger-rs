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

//! A connection to a WiredTiger database.

use crate::error::Result;
use crate::session::Session;
use std::ffi::{CStr, CString};
use std::path::Path;
use std::ptr;
use wiredtiger_sys::{wiredtiger_open, WT_CONNECTION, WT_SESSION};

pub struct Connection {
    inner: Option<*mut WT_CONNECTION>,
}

unsafe impl Send for Connection {}
unsafe impl Sync for Connection {}

macro_rules! conn_api {
    ($conn: ident, $api: ident) => {
        unsafe {
            let conn = $conn.inner.expect("connection is null");
            let api = (*conn).$api.expect("null function pointer");
            (conn, api)
        }
    };
}

impl Connection {
    pub fn open<P: AsRef<Path>, C: AsRef<str>>(home: P, config: C) -> Result<Connection> {
        let home = home.as_ref();
        let c_home = CString::new(home.to_string_lossy().as_bytes()).unwrap();

        let config = config.as_ref();
        let c_config = CString::new(config.as_bytes()).unwrap();

        let mut conn: *mut WT_CONNECTION = ptr::null_mut();
        unsafe {
            wt_try!(wiredtiger_open(
                c_home.as_ptr(),
                ptr::null_mut(),
                c_config.as_ptr(),
                &mut conn as *mut *mut WT_CONNECTION
            ));
        }

        assert!(!conn.is_null());
        Ok(unsafe { Connection::new_unchecked(conn) })
    }

    #[inline]
    unsafe fn new_unchecked(conn: *mut WT_CONNECTION) -> Self {
        debug_assert!(!conn.is_null());
        Self { inner: Some(conn) }
    }

    pub fn close<T: AsRef<str>>(&mut self, config: T) -> Result<()> {
        if let Some(conn) = self.inner {
            debug_assert!(!conn.is_null());
            unsafe {
                let close = (*conn).close.unwrap();

                let config = config.as_ref();
                if config.is_empty() {
                    wt_try!(close(conn, std::ptr::null()));
                } else {
                    let c_config = CString::new(config.as_bytes()).unwrap();
                    wt_try!(close(conn, c_config.as_ptr()));
                }
            }
            self.inner.take();
        }

        Ok(())
    }

    #[inline]
    pub fn get_home(&self) -> &str {
        let (conn, get_home) = conn_api!(self, get_home);
        unsafe {
            let home = get_home(conn);
            CStr::from_ptr(home).to_str().unwrap()
        }
    }

    #[inline]
    pub fn is_new(&self) -> bool {
        let (conn, is_new) = conn_api!(self, is_new);
        unsafe { is_new(conn) != 0 }
    }

    pub fn open_session<C: AsRef<str>>(&self, config: C) -> Result<Session> {
        let (conn, open_session) = conn_api!(self, open_session);
        let c_config = CString::new(config.as_ref().as_bytes()).unwrap();
        let mut session: *mut WT_SESSION = ptr::null_mut();
        unsafe {
            wt_try!(open_session(
                conn,
                ptr::null_mut(),
                c_config.as_ptr(),
                &mut session as *mut *mut WT_SESSION
            ));
            assert!(!session.is_null());
            Ok(Session::new_unchecked(session))
        }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        let result = self.close("");
        match result {
            Ok(_) => (),
            Err(error) => eprintln!("error happened when auto close connection: {}", error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils;

    #[test]
    fn test_connection() {
        let home = "target/wt_connection";
        test_utils::ensure_wt_home(home, true);
        let mut conn = Connection::open(home, "create").unwrap();
        assert_eq!(conn.get_home(), home);
        assert!(conn.is_new());
        conn.close("").unwrap();
    }
}
