// Copyright (C) 2016  ParadoxSpiral
//
// This file is part of mpv-rs.
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA

use libc;
use enum_primitive::FromPrimitive;

use super::*;
use super::super::raw::*;

use std::path::Path;
use std::ffi::{CStr, /*CString*/};

#[cfg(unix)]
use std::ffi::OsStr;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

// Cast `&mut Data` so that libmpv can use it.
macro_rules! data_ptr {
    ($data:expr) => (
        #[allow(match_ref_pats)]
        match $data {
            &mut Data::Flag(ref mut v) =>
                v as *mut bool as *mut libc::c_void,
            &mut Data::Int64(ref mut v) =>
                v as *mut libc::int64_t as *mut libc::c_void,
            &mut Data::Double(ref mut v) =>
                v as *mut libc::c_double as *mut libc::c_void,
            _ => unreachable!(),
        }
    )
}

pub(crate) fn mpv_err<T>(ret: T, err_val: libc::c_int) -> Result<T, Error> {
    debug_assert!(err_val <= 0);
    if err_val == 0 {
        Ok(ret)
    } else {
        Err(Error::Mpv(MpvError::from_i32(err_val).unwrap()))
    }
}

pub(crate) fn property_from_raw(raw: *mut libc::c_void) -> (String, Data) {
    debug_assert!(!raw.is_null());
    let raw = unsafe { &mut *(raw as *mut MpvEventProperty) };
    (
        unsafe { CStr::from_ptr(raw.name).to_str().unwrap().into() },
        Data::from_raw(raw.format, raw.data)
    )
}

pub(crate) fn mpv_cstr_to_string(cstr: &CStr) -> String {
    let data;
    #[cfg(windows)] {
        // Mpv returns all strings on windows in UTF-8.
        data = cstr.to_str().unwrap().to_owned();
    }
    #[cfg(unix)] {
        data = OsStr::from_bytes(cstr.to_bytes()).to_string_lossy().into_owned();
    }
    #[cfg(all(not(unix), not(windows)))] {
        // Hope that all is well
        data = String::from_utf8_lossy(cstr.to_bytes()).into_owned();
    }
    data
}

#[allow(missing_docs)]
/// Subset of `MpvFormat` used by the public API.
pub enum Format {
    String,
    OsdString,
    Flag,
    Int64,
    Double,
}

impl Format {
    pub(crate) fn as_mpv_format(&self) -> MpvFormat {
        match *self {
            Format::String => MpvFormat::String,
            Format::OsdString => MpvFormat::OsdString,
            Format::Flag => MpvFormat::Flag,
            Format::Int64 => MpvFormat::Int64,
            Format::Double => MpvFormat::Double,
        }
    }
}

impl MpvError {
    pub(crate) fn as_val(&self) -> libc::c_int {
        *self as libc::c_int
    }

    #[inline]
    /// Returns the associated error string.
    pub fn error_string(&self) -> &str {
        let raw = unsafe { mpv_error_string(self.as_val()) };
        unsafe { CStr::from_ptr(raw) }.to_str().unwrap()
    }
}

impl MpvFormat {
    pub(crate) fn as_val(self) -> libc::c_int {
        self as _
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// How a `File` is inserted into the playlist.
pub enum FileState {
    /// Replace the current track.
    Replace,
    /// Append to the current playlist.
    Append,
    /// If current playlist is empty: play, otherwise append to playlist.
    AppendPlay,
}

impl FileState {
    pub(crate) fn val(&self) -> &str {
        match *self {
            FileState::Replace => "replace",
            FileState::Append => "append",
            FileState::AppendPlay => "append-play",
        }
    }
}

#[derive(Clone, Debug)]
/// A command that can be executed by `Mpv`.
pub struct Command<'a> {
    pub(crate) name: &'a str,
    pub(crate) args: &'a [&'a str],
}

impl<'a> Command<'a> {
    #[inline]
    /// Create a new `MpvCommand`.
    pub fn new(name: &'a str, args: &'a [&'a str]) -> Command<'a> {
        Command {
            name: name,
            args: args,
        }
    }
}

#[derive(Clone, Debug)]
/// Data needed for `PlaylistOp::Loadfiles`.
pub struct File<'a> {
    pub(crate) path: &'a Path,
    pub(crate) state: FileState,
    pub(crate) options: Option<&'a str>,
}

impl<'a> File<'a> {
    #[inline]
    /// Create a new `File`.
    pub fn new(path: &'a Path, state: FileState, opts: Option<&'a str>) -> File<'a> {
        File {
            path: path,
            state: state,
            options: opts,
        }
    }
}
