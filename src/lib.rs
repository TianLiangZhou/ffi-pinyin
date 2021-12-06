use pinyin::{ToPinyin, PinyinStrIter, Pinyin, PinyinMultiStrIter, ToPinyinMulti};
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int, c_schar};
use std::{mem, ptr};


#[derive(Copy, Clone)]
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


const PLAIN: fn(Option<Pinyin>) -> &'static str = |t: Option<Pinyin> | {
    match t {
        Some(py) => py.plain(),
        None => "-"
    }
};

const TONE: fn(Option<Pinyin>) -> &'static str = |t: Option<Pinyin> | {
    match t {
        Some(py) => py.with_tone(),
        None => "-"
    }
};

const LETTER: fn(Option<Pinyin>) -> &'static str = |t: Option<Pinyin> | {
    match t {
        Some(py) => py.first_letter(),
        None => "-"
    }
};


#[no_mangle]
pub extern "C" fn plain(str: *const c_char, is_convert: c_int) -> *mut c_char {
    let str = const_to_str(str);
    let pinyin = str.to_pinyin();
    let mut m: String  = String::new();
    if is_convert == 1 {
        m = pinyin.map(PLAIN).collect::<Vec<&'static str>>().join(" ")
    } else {
        m = pinyin.enumerate().map(|(i, single)| -> String {
            match single {
                None => {
                    str.chars().nth(i).unwrap().to_string()
                }
                Some(py) => py.plain().to_string()
            }

        }).collect::<Vec<String>>().join(" ");
    }
    return CString::new(m).unwrap().into_raw();
}

#[no_mangle]
pub extern "C" fn tone(str: *const c_char, is_convert: c_int) -> *mut c_char {
    let str = const_to_str(str);
    let pinyin = str.to_pinyin();
    let mut m: String  = String::new();
    if is_convert == 1 {
        m = pinyin.map(TONE).collect::<Vec<&'static str>>().join(" ")
    } else {
        m = pinyin.enumerate().map(|(i, single)| -> String {
            match single {
                None => {
                    str.chars().nth(i).unwrap().to_string()
                }
                Some(py) => py.with_tone().to_string()
            }

        }).collect::<Vec<String>>().join(" ");
    }
    return CString::new(m).unwrap().into_raw();
}

#[no_mangle]
pub extern "C" fn letter(str: *const c_char) -> *mut c_char {
    let m: String = const_to_str(str).to_pinyin().map(LETTER).collect::<Vec<&'static str>>().join(" ");
    return CString::new(m).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn plain_array(str: *const c_char, is_convert: c_int) -> *mut PinyinArray {
    let str = const_to_str(str);
    let pinyin = str.to_pinyin();
    let mut vec = pinyin.enumerate().map(|(index, word) | {
        match word {
            None => {
                if is_convert == 1 {
                    return PinyinStr::from_string(String::from("-"));
                }
                return PinyinStr::from_string(str.chars().nth(index).unwrap().to_string())
            }
            Some(pinyin) => {
                PinyinStr::from_string(pinyin.plain().to_string())
            }
        }
    }).collect::<Vec<PinyinStr>>();
    let len = vec.len();
    let ptr = vec.as_mut_ptr();
    mem::forget(vec);
    Box::into_raw(Box::new(PinyinArray{
        array: ptr,
        len,
    }))
}

#[no_mangle]
pub extern "C" fn tone_array(str: *const c_char, is_convert: c_int) -> *mut PinyinArray {
    let str = const_to_str(str);
    let pinyin = str.to_pinyin();
    let mut vec = pinyin.enumerate().map(|(index, word) | {
        match word {
            None => {
                if is_convert == 1 {
                    return PinyinStr::from_string(String::from("-"));
                }
                return PinyinStr::from_string(str.chars().nth(index).unwrap().to_string())
            }
            Some(pinyin) => {
                PinyinStr::from_string(pinyin.with_tone().to_string())
            }
        }
    }).collect::<Vec<PinyinStr>>();
    let len = vec.len();
    let ptr = vec.as_mut_ptr();
    mem::forget(vec);
    Box::into_raw(Box::new(PinyinArray{
        array: ptr,
        len,
    }))
}

#[no_mangle]
pub extern "C" fn tone_multi(str: *const c_char) -> *mut c_char {
    let mut vec:Vec<String> = Vec::new();
    let iter = const_to_str(str).to_pinyin_multi();
    for item in iter {
        if let Some(item) = item {
            if item.count() > 1 {
                vec.push(
                    item.into_iter().map(|x| {
                        x.with_tone()
                    }).collect::<Vec<&'static str>>().join(":")
                )
            } else {
                vec.push(item.get(0).with_tone().to_string())
            }
        } else {
            vec.push(String::from("-"));
        }
    }
    return CString::new(vec.join(" ")).unwrap().into_raw()
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


fn to_pinyin<'a>(str: *const c_char) -> PinyinStrIter<'a> {
    let text = unsafe { CStr::from_ptr(str) }.to_str().unwrap();
    text.to_pinyin()
}

fn to_pinyin_multi<'a>(str: *const c_char) -> PinyinMultiStrIter<'a> {
    let text = unsafe { CStr::from_ptr(str) }.to_str().unwrap();
    text.to_pinyin_multi()
}

fn const_to_str(str: *const c_char) -> &'static str {
    return unsafe { CStr::from_ptr(str) }.to_str().unwrap();
}

#[cfg(test)]
mod tests {
    use pinyin::{ToPinyin, ToPinyinMulti, Pinyin};
    use crate::to_pinyin;
    use std::os::raw::c_char;
    use std::ffi::{CString, CStr};

    #[test]
    fn it_works() {

        let str = "中国人。。。";

        let mut chars = str.chars();

        let pinyin = str.to_pinyin();

        let m = pinyin.enumerate().map(|(i, single)| -> String {
            match single {
                None => {
                    str.chars().nth(i).unwrap().to_string()
                }
                Some(py) => py.plain().to_string()
            }

        }).collect::<Vec<String>>().join(" ");

        println!("m = {}", m);

        let p = CString::new("中国人").unwrap().into_raw();
        let m: String = to_pinyin(p).map(|t| {
           return match t {
                Some(py) => py.plain(),
                None => "-"
            }
        }).collect::<Vec<&'static str>>().join(" ");
        assert_eq!("zhong guo ren", m);

        let iter = "中国人。。。".to_pinyin_multi();
        let mut vec:Vec<String> = Vec::new();
        for item in iter {
            if let Some(item) = item {
                if item.count() > 1 {
                    vec.push(
                        item.into_iter().map(|x| {
                            x.with_tone()
                        }).collect::<Vec<&'static str>>().join(":")
                    )
                } else {
                    vec.push(item.get(0).with_tone().to_string())
                }
            } else {
                vec.push(String::from("-"));
            }
        }
        println!("{}", vec.join(" "));
        let str = "中国人。我fdafd😄";
        let pinyin = str.to_pinyin();
        let mut i = 0;
        for x in pinyin {
            match x {
                None => {
                    println!(" index = {}", str.chars().nth(i).unwrap())
                }
                Some(py) => {
                    println!(" word = {}", py.plain())
                }
            }
            i = i + 1;
        }
    }
}


