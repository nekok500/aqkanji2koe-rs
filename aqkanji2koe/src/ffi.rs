use std::ffi::c_void;

use libc::{c_char, c_int};

#[link(name = "AqKanji2Koe", kind = "dylib")]
extern "C" {
    pub fn AqKanji2Koe_Create(pathDic: *const c_char, pErr: *const c_int) -> *const c_void;
    pub fn AqKanji2Koe_Convert(
        hAqKanji2Koe: *const c_void,
        kanji: *const c_char,
        koe: *const c_char,
        nBufKoe: c_int,
    ) -> c_int;
    pub fn AqKanji2Koe_SetDevKey(key: *const c_char) -> c_int;
    pub fn AqKanji2Koe_Release(hAqKanji2Koe: *const c_void);
}
