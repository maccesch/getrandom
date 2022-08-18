// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation for OpenBSD
use crate::{util_libc::last_os_error, Error};

pub unsafe fn getrandom_inner(mut dst: *mut u8, mut len: usize) -> Result<(), Error> {
    // getentropy(2) was added in OpenBSD 5.6, so we can use it unconditionally.
    while len != 0 {
        let chunk_len = core::cmp::min(len, 256);
        // TODO: use `cast` on MSRV bump to 1.38
        let ret = libc::getentropy(dst as *mut libc::c_void, chunk_len);
        if ret == -1 {
            return Err(last_os_error());
        }
        dst = dst.add(chunk_len);
        len -= chunk_len;
    }
    Ok(())
}
