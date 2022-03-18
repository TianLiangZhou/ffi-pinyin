use pinyin::{Pinyin, PinyinMulti, ToPinyin, ToPinyinMulti};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_schar, c_uchar};
use std::thread::spawn;
use std::{mem, ptr};

#[repr(C)]
pub struct PinyinArray {
    pub len: usize,
    pub array: *mut PinyinStr,
}

#[repr(C)]
pub struct PinyinStr {
    pub len: usize,
    pub data: *mut c_char,
    pub owned: bool,
    pub convert: u8,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Mode {
    Plain = 1,
    Tone = 2,
    Letter = 3,
    ToneNum = 4,
    ToneNumEnd = 5,
}

impl Default for PinyinStr {
    fn default() -> Self {
        Self {
            data: ptr::null_mut(),
            len: 0,
            owned: false,
            convert: 0,
        }
    }
}

impl PinyinStr {
    pub fn from_string(mut s: String, convert: u8) -> Self {
        s.shrink_to_fit();
        let rv = Self {
            data: s.as_ptr() as *mut c_char,
            len: s.len(),
            owned: true,
            convert,
        };
        mem::forget(s);
        rv
    }

    pub unsafe fn free(&mut self) {
        if self.owned && !self.data.is_null() {
            String::from_raw_parts(self.data as *mut _, self.len, self.len);
            self.data = ptr::null_mut();
            self.len = 0;
            self.owned = false;
        }
    }
}

impl Drop for PinyinStr {
    fn drop(&mut self) {
        unsafe {
            self.free();
        }
    }
}

fn match_mode(py: Pinyin, mode: Mode) -> String {
    match mode {
        Mode::Plain => py.plain().to_string(),
        Mode::Tone => py.with_tone().to_string(),
        Mode::Letter => py.first_letter().to_string(),
        Mode::ToneNum => py.with_tone_num().to_string(),
        Mode::ToneNumEnd => py.with_tone_num_end().to_string(),
    }
}

/// **
///
/// **
fn to_convert(
    str: &'static str,
    is_ignore_unknown_char: bool,
    separator: char,
    not_split_unknown_char: bool,
    mode: Mode,
) -> String {
    let chars = str.chars().collect::<Vec<char>>();
    let mut unknown = String::new();
    let mut vec: Vec<String> = Vec::new();
    for word in str.to_pinyin().enumerate() {
        match word.1 {
            None => {
                if is_ignore_unknown_char {
                    continue;
                }
                let word_char = chars.get(word.0).unwrap();
                if not_split_unknown_char {
                    unknown.push(*word_char);
                    if word.0 == chars.len() - 1 {
                        vec.push(unknown.clone());
                        unknown.clear();
                    }
                } else {
                    vec.push(word_char.to_string());
                }
            }
            Some(py) => {
                if !is_ignore_unknown_char && not_split_unknown_char && unknown.len() > 0 {
                    vec.push(unknown.clone());
                    unknown.clear();
                }
                vec.push(match_mode(py, mode));
            }
        }
    }
    vec.join(separator.encode_utf8(&mut [0; 4]))
}

///
///
///
fn to_convert_multi(
    str: &'static str,
    is_ignore_unknown_char: bool,
    separator: char,
    not_split_unknown_char: bool,
    mode: Mode,
) -> String {
    let chars = str.chars().collect::<Vec<char>>();
    let mut unknown = String::new();
    let mut vec: Vec<String> = Vec::new();
    for word in str.to_pinyin_multi().enumerate() {
        match word.1 {
            None => {
                if is_ignore_unknown_char {
                    continue;
                }
                let word_char = chars.get(word.0).unwrap();
                if not_split_unknown_char {
                    unknown.push(*word_char);
                    if word.0 == chars.len() - 1 {
                        vec.push(unknown.clone());
                        unknown.clear();
                    }
                } else {
                    vec.push(word_char.to_string());
                }
            }
            Some(multi) => {
                if !is_ignore_unknown_char && not_split_unknown_char && unknown.len() > 0 {
                    vec.push(unknown.clone());
                    unknown.clear();
                }
                vec.push(
                    multi
                        .into_iter()
                        .map(|py| match_mode(py, mode))
                        .collect::<Vec<String>>()
                        .join(":"),
                );
            }
        }
    }
    vec.join(separator.encode_utf8(&mut [0; 4]))
}

fn to_convert_array(
    str: &'static str,
    is_ignore_unknown_char: bool,
    is_multi: bool,
    not_split_unknown_char: bool,
    mode: Mode,
) -> Vec<PinyinStr> {
    let chars = str.chars().collect::<Vec<char>>();
    let mut vec: Vec<PinyinStr> = Vec::new();
    let mut unknown = String::new();
    if is_multi {
        for word in str.to_pinyin_multi().enumerate() {
            match word.1 {
                None => {
                    if is_ignore_unknown_char {
                        continue;
                    }
                    let word_char = chars.get(word.0).unwrap();
                    if not_split_unknown_char {
                        unknown.push(*word_char);
                        if word.0 == chars.len() - 1 {
                            vec.push(PinyinStr::from_string(unknown.clone(), 0));
                            unknown.clear();
                        }
                    } else {
                        vec.push(PinyinStr::from_string(word_char.to_string(), 0))
                    }
                }
                Some(multi) => {
                    if !is_ignore_unknown_char && not_split_unknown_char && unknown.len() > 0 {
                        vec.push(PinyinStr::from_string(unknown.clone(), 0));
                        unknown.clear();
                    }
                    vec.push(PinyinStr::from_string(
                        multi
                            .into_iter()
                            .map(|py| match_mode(py, mode))
                            .collect::<Vec<String>>()
                            .join(":"),
                        1,
                    ))
                }
            }
        }
    } else {
        for word in str.to_pinyin().enumerate() {
            match word.1 {
                None => {
                    if is_ignore_unknown_char {
                        continue;
                    }
                    let word_char = chars.get(word.0).unwrap();
                    if not_split_unknown_char {
                        unknown.push(*word_char);
                        if word.0 == chars.len() - 1 {
                            vec.push(PinyinStr::from_string(unknown.clone(), 0));
                            unknown.clear();
                        }
                    } else {
                        vec.push(PinyinStr::from_string(word_char.to_string(), 0))
                    }
                }
                Some(py) => {
                    if !is_ignore_unknown_char && not_split_unknown_char && unknown.len() > 0 {
                        vec.push(PinyinStr::from_string(unknown.clone(), 0));
                        unknown.clear();
                    }
                    vec.push(PinyinStr::from_string(match_mode(py, mode), 1))
                }
            }
        }
    }
    vec
}

fn to_array_pointer(mut vec: Vec<PinyinStr>) -> *mut PinyinArray {
    let len = vec.len();
    let ptr = vec.as_mut_ptr();
    mem::forget(vec);
    Box::into_raw(Box::new(PinyinArray { array: ptr, len }))
}

#[no_mangle]
pub extern "C" fn to_pinyin(
    str: *const c_char,
    is_ignore_unknown_char: c_int,
    is_multi: c_int,
    separator: c_uchar,
    not_split_unknown_char: c_int,
    mode: Mode,
) -> *mut c_char {
    let pinyin_str = if is_multi == 1 {
        to_convert_multi(
            const_to_str(str),
            is_ignore_unknown_char == 1,
            separator as char,
            not_split_unknown_char == 1,
            mode,
        )
    } else {
        to_convert(
            const_to_str(str),
            is_ignore_unknown_char == 1,
            separator as char,
            not_split_unknown_char == 1,
            mode,
        )
    };
    return CString::new(pinyin_str).unwrap().into_raw();
}

#[no_mangle]
pub extern "C" fn to_pinyin_array(
    str: *const c_char,
    is_ignore_unknown_char: c_int,
    is_multi: c_int,
    not_split_unknown_char: c_int,
    mode: Mode,
) -> *mut PinyinArray {
    let vec = to_convert_array(
        const_to_str(str),
        is_ignore_unknown_char == 1,
        is_multi == 1,
        not_split_unknown_char == 1,
        mode,
    );
    return to_array_pointer(vec);
}

#[no_mangle]
pub extern "C" fn free_pointer(ptr: *mut c_char) {
    unsafe {
        if ptr.is_null() {
            // No data there, already freed probably.
            return;
        }
        // Here we reclaim ownership of the data the pointer points to, to free the memory properly.
        CString::from_raw(ptr);
    }
}

#[no_mangle]
pub unsafe extern "C" fn free_array(array: *mut PinyinArray) {
    if !array.is_null() {
        Vec::from_raw_parts((*array).array, (*array).len, (*array).len);
        Box::from_raw(array);
    }
}

fn const_to_str(str: *const c_char) -> &'static str {
    return unsafe { CStr::from_ptr(str) }.to_str().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::{to_convert, to_convert_array, to_convert_multi, Mode};
    use std::ffi::CStr;
    use std::os::raw::c_char;

    #[test]
    fn it_works() {
        let str = "测试中文汉字转拼音。😊，rust yyds加上不能识别的结尾。。。";

        let pinyin_str = to_convert(str, false, '-', true, Mode::Plain);
        println!("plain: {}", pinyin_str);

        let pinyin_str = to_convert_multi(str, true, '-', true, Mode::Tone);
        println!("plain: {}", pinyin_str);

        let str = "最快的php汉字转拼音rust库。😊，欧耶rust yyds";
        let pinyin_vec = to_convert_array(str, false, true, true, Mode::ToneNumEnd);
        for x in pinyin_vec {
            println!(
                "char : {}",
                unsafe { CStr::from_ptr(x.data) }.to_str().unwrap()
            );
        }
    }
}
