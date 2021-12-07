use pinyin::{ToPinyin, Pinyin, ToPinyinMulti};
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int};
use std::{mem, ptr};


#[repr(C)]
pub struct PinyinArray {
    pub len: usize,
    pub array: *mut PinyinStr
}

#[repr(C)]
pub struct PinyinStr {
    pub len: usize,
    pub data: *mut c_char,
    pub owned: bool,
}

impl Default for PinyinStr {
    fn default() -> Self {
        Self {
            data: ptr::null_mut(),
            len: 0,
            owned: false
        }
    }
}

impl PinyinStr {
    pub fn from_string(mut s: String) -> Self {
        s.shrink_to_fit();
        let rv = Self {
            data: s.as_ptr() as *mut c_char,
            len: s.len(),
            owned: true,
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

#[derive(Copy, Clone)]
enum Mode {
    Plain,
    Tone,
    Letter,
    ToneNum,
    ToneNumEnd
}


fn match_mode(py: Pinyin, mode: Mode) -> String {
    match mode {
        Mode::Plain => py.plain().to_string(),
        Mode::Tone => py.with_tone().to_string(),
        Mode::Letter => py.first_letter().to_string(),
        Mode::ToneNum => py.with_tone_num().to_string(),
        Mode::ToneNumEnd => py.with_tone_num_end().to_string()
    }
}

/// **
///
/// **
fn to_pinyin(str: &'static str, is_convert: bool, is_multi: bool, mode: Mode) -> String {
    let chars = str.chars().collect::<Vec<char>>();
    let convert = |index: usize| {
        if is_convert {
            return String::from("-");
        }
        return chars.get(index).unwrap().to_string();
    };
    return if is_multi {
        str.to_pinyin_multi().enumerate().map(|(index, word)| {
            match word {
                None => convert(index),
                Some(multi) => {
                    multi.into_iter().map(|py| {
                        match_mode(py, mode)
                    }).collect::<Vec<String>>().join(":")
                }
            }
        }).collect::<Vec<String>>().join(" ")
    } else {
        str.to_pinyin().enumerate().map(|(index, word)| {
            match word {
                None => convert(index),
                Some(py) => match_mode(py, mode)
            }
        }).collect::<Vec<String>>().join(" ")
    };
}

fn to_pinyin_array(str: &'static str, is_convert: bool, is_multi: bool, mode: Mode) -> Vec<PinyinStr>{
    let chars = str.chars().collect::<Vec<char>>();
    let convert = |index: usize| {
        if is_convert {
            return PinyinStr::from_string(String::from("-"));
        }
        return PinyinStr::from_string(chars.get(index).unwrap().to_string())
    };
    return if is_multi {
        str.to_pinyin_multi().enumerate().map(|(index, word)| {
            match word {
                None => convert(index),
                Some(multi) => {
                    PinyinStr::from_string(multi.into_iter().map(|py| {
                        match_mode(py, mode)
                    }).collect::<Vec<String>>().join(":"))
                }
            }
        }).collect::<Vec<PinyinStr>>()
    } else {
        str.to_pinyin().enumerate().map(|(index, word) | {
            match word {
                None => convert(index),
                Some(py) => PinyinStr::from_string(match_mode(py, mode))
            }
        }).collect::<Vec<PinyinStr>>()
    };
}

fn to_array_pointer(mut vec: Vec<PinyinStr>) -> *mut PinyinArray {
    let len = vec.len();
    let ptr = vec.as_mut_ptr();
    mem::forget(vec);
    Box::into_raw(Box::new(PinyinArray{
        array: ptr,
        len,
    }))
}


#[no_mangle]
pub extern "C" fn plain(str: *const c_char, is_convert: c_int, is_multi: c_int) -> *mut c_char {
    let pinyin_str = to_pinyin(const_to_str(str), is_convert == 1, is_multi == 1, Mode::Plain);
    return CString::new(pinyin_str).unwrap().into_raw();
}

#[no_mangle]
pub extern "C" fn tone(str: *const c_char, is_convert: c_int, is_multi: c_int) -> *mut c_char {
    let pinyin_str = to_pinyin(const_to_str(str), is_convert == 1, is_multi == 1,Mode::Tone);
    return CString::new(pinyin_str).unwrap().into_raw();
}

#[no_mangle]
pub extern "C" fn tone_num(str: *const c_char, is_convert: c_int, is_multi: c_int) -> *mut c_char {
    let pinyin_str = to_pinyin(const_to_str(str), is_convert == 1, is_multi == 1,Mode::ToneNum);
    return CString::new(pinyin_str).unwrap().into_raw();
}

#[no_mangle]
pub extern "C" fn tone_num_end(str: *const c_char, is_convert: c_int, is_multi: c_int) -> *mut c_char {
    let pinyin_str = to_pinyin(const_to_str(str), is_convert == 1, is_multi == 1,Mode::ToneNumEnd);
    return CString::new(pinyin_str).unwrap().into_raw();
}

#[no_mangle]
pub extern "C" fn letter(str: *const c_char, is_convert: c_int, is_multi: c_int) -> *mut c_char {
    let pinyin_str = to_pinyin(const_to_str(str), is_convert == 1, is_multi == 1,Mode::Letter);
    return CString::new(pinyin_str).unwrap().into_raw();
}

#[no_mangle]
pub extern "C" fn plain_array(str: *const c_char, is_convert: c_int, is_multi: c_int) -> *mut PinyinArray {
    let vec = to_pinyin_array(const_to_str(str), is_convert == 1, is_multi == 1, Mode::Plain);
    return to_array_pointer(vec);
}

#[no_mangle]
pub extern "C" fn tone_array(str: *const c_char, is_convert: c_int, is_multi: c_int) -> *mut PinyinArray {
    let vec = to_pinyin_array(const_to_str(str), is_convert == 1, is_multi == 1, Mode::Tone);
    return to_array_pointer(vec);
}

#[no_mangle]
pub extern "C" fn tone_num_array(str: *const c_char, is_convert: c_int, is_multi: c_int) -> *mut PinyinArray {
    let vec = to_pinyin_array(const_to_str(str), is_convert == 1, is_multi == 1, Mode::ToneNum);
    return to_array_pointer(vec);
}

#[no_mangle]
pub extern "C" fn tone_num_end_array(str: *const c_char, is_convert: c_int, is_multi: c_int) -> *mut PinyinArray {
    let vec = to_pinyin_array(const_to_str(str), is_convert == 1, is_multi == 1, Mode::ToneNumEnd);
    return to_array_pointer(vec);
}

#[no_mangle]
pub extern "C" fn letter_array(str: *const c_char, is_convert: c_int, is_multi: c_int) -> *mut PinyinArray {
    let vec = to_pinyin_array(const_to_str(str), is_convert == 1, is_multi == 1, Mode::Letter);
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
    use std::ffi::CStr;
    use crate::{Mode, to_pinyin, to_pinyin_array};

    #[test]
    fn it_works() {

        let str = "我是中国人。😊，欧耶";

        let pinyin_str = to_pinyin(str, true, true, Mode::ToneNumEnd);
        println!("plain: {}", pinyin_str);


        let str = "最快的汉字转拼音库。😊，欧耶";
        let pinyin_vec = to_pinyin_array(str, true, true, Mode::ToneNumEnd);
        for x in pinyin_vec {
            println!("char : {}", unsafe {CStr::from_ptr(x.data)}.to_str().unwrap());
        }

    }
}


