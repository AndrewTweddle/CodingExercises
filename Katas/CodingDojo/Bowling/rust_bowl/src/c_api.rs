use crate::pattern_scorer;
use std::ffi::CStr;
use std::os::raw::c_char;

#[repr(i32)]
enum ThrowsErrorCode {
    NullPointer = -1,
    InvalidUtf8 = -2,
    InvalidPattern = -3,
}

#[no_mangle]
pub extern "C" fn score_with_rust(throws: *const c_char) -> i32 {
    if throws.is_null() {
        return ThrowsErrorCode::NullPointer as i32;
    }

    let throws_c_str = unsafe { CStr::from_ptr(throws) };

    throws_c_str
        .to_str()
        .map_or(ThrowsErrorCode::InvalidUtf8 as i32, |throws_str| {
            pattern_scorer::score_bowling_throws(throws_str)
                .map_or(ThrowsErrorCode::InvalidPattern as i32, |score_u16| {
                    score_u16 as i32
                })
            // TODO: map each of the BowlingScorerError values to different error codes
        })
}

// TODO: Add unit tests for each of these error conditions
