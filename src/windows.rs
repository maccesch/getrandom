// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::Error;
use core::{ffi::c_void, num::NonZeroU32, ptr};

const BCRYPT_USE_SYSTEM_PREFERRED_RNG: u32 = 0x00000002;

#[link(name = "bcrypt")]
extern "system" {
    fn BCryptGenRandom(
        hAlgorithm: *mut c_void,
        pBuffer: *mut u8,
        cbBuffer: u32,
        dwFlags: u32,
    ) -> u32;
}

pub unsafe fn getrandom_inner(mut dst: *mut u8, mut len: usize) -> Result<(), Error> {
    // Prevent overflow of u32
    while len != 0 {
        let chunk_len = core::cmp::min(len, u32::max_value() as usize);
        // BCryptGenRandom was introduced in Windows Vista
        let ret = BCryptGenRandom(
            ptr::null_mut(),
            dst,
            chunk_len as u32,
            BCRYPT_USE_SYSTEM_PREFERRED_RNG,
        );
        // NTSTATUS codes use the two highest bits for severity status.
        if ret >> 30 == 0b11 {
            // We zeroize the highest bit, so the error code will reside
            // inside the range designated for OS codes.
            let code = ret ^ (1 << 31);
            // SAFETY: the second highest bit is always equal to one,
            // so it's impossible to get zero. Unfortunately the type
            // system does not have a way to express this yet.
            let code = NonZeroU32::new_unchecked(code);
            return Err(Error::from(code));
        }

        dst = dst.add(chunk_len);
        len -= chunk_len;
    }
    Ok(())
}
