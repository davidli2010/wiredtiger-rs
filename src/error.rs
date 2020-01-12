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

//! WiredTiger error handling.

use std::borrow::Cow;
use std::ffi::CStr;
use std::fmt::Formatter;
use wiredtiger_sys;

/// WiredTiger's Error.
#[derive(Debug)]
pub enum Error {
    System(i32),
    Rollback,
    DuplicateKey,
    GeneralError,
    NotFound,
    Panic,
    Restart,
    RunRecovery,
    CacheFull,
    PrepareConflict,
    TrySalvage,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let strerror = |error| unsafe { wt_strerror(error) };

        match self {
            Error::System(i) => write!(f, "SYSTEM({}): {}", i, strerror(*i)),
            Error::Rollback => write!(f, "{}", strerror(wiredtiger_sys::WT_ROLLBACK)),
            Error::DuplicateKey => write!(f, "{}", strerror(wiredtiger_sys::WT_DUPLICATE_KEY)),
            Error::GeneralError => write!(f, "{}", strerror(wiredtiger_sys::WT_ERROR)),
            Error::NotFound => write!(f, "{}", strerror(wiredtiger_sys::WT_NOTFOUND)),
            Error::Panic => write!(f, "{}", strerror(wiredtiger_sys::WT_PANIC)),
            Error::Restart => write!(f, "{}", strerror(wiredtiger_sys::WT_RESTART)),
            Error::RunRecovery => write!(f, "{}", strerror(wiredtiger_sys::WT_RUN_RECOVERY)),
            Error::CacheFull => write!(f, "{}", strerror(wiredtiger_sys::WT_CACHE_FULL)),
            Error::PrepareConflict => {
                write!(f, "{}", strerror(wiredtiger_sys::WT_PREPARE_CONFLICT))
            }
            Error::TrySalvage => write!(f, "{}", strerror(wiredtiger_sys::WT_TRY_SALVAGE)),
        }
    }
}

impl From<i32> for Error {
    fn from(value: i32) -> Self {
        match value {
            wiredtiger_sys::WT_ROLLBACK => Error::Rollback,
            wiredtiger_sys::WT_DUPLICATE_KEY => Error::DuplicateKey,
            wiredtiger_sys::WT_ERROR => Error::GeneralError,
            wiredtiger_sys::WT_NOTFOUND => Error::NotFound,
            wiredtiger_sys::WT_PANIC => Error::Panic,
            wiredtiger_sys::WT_RESTART => Error::Restart,
            wiredtiger_sys::WT_RUN_RECOVERY => Error::RunRecovery,
            wiredtiger_sys::WT_CACHE_FULL => Error::CacheFull,
            wiredtiger_sys::WT_PREPARE_CONFLICT => Error::PrepareConflict,
            wiredtiger_sys::WT_TRY_SALVAGE => Error::TrySalvage,
            v if value > 0 => Error::System(v),
            v => panic!(format!("unknown error code: {}", v)),
        }
    }
}

unsafe fn wt_strerror<'a>(error: i32) -> Cow<'a, str> {
    let cstr_error = wiredtiger_sys::wiredtiger_strerror(error);
    CStr::from_ptr(cstr_error).to_string_lossy()
}

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! wt_try {
    ($expr:expr) => {
        let errcode: i32 = $expr;
        if errcode != 0 {
            return $crate::error::Result::Err($crate::error::Error::from(errcode));
        }
    };
    ($expr:expr,) => {
        $crate::wt_try!($expr)
    };
}
