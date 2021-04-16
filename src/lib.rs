use pinyin::{ToPinyin, PinyinStrIter, Pinyin, PinyinMultiStrIter, ToPinyinMulti};
use std::ffi::{CString, CStr};
use std::os::raw::c_char;


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
pub extern "C" fn plain(str: *const c_char) -> *mut c_char {
    let m: String = to_pinyin(str).map(PLAIN).collect::<Vec<&'static str>>().join(" ");
    return CString::new(m).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn tone(str: *const c_char) -> *mut c_char {
    let m: String = to_pinyin(str).map(TONE).collect::<Vec<&'static str>>().join(" ");
    return CString::new(m).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn letter(str: *const c_char) -> *mut c_char {
    let m: String = to_pinyin(str).map(LETTER).collect::<Vec<&'static str>>().join(" ");
    return CString::new(m).unwrap().into_raw()
}



#[no_mangle]
pub extern "C" fn tone_multi(str: *const c_char) -> *mut c_char {
    let mut vec:Vec<String> = Vec::new();
    let iter = to_pinyin_multi(str);
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

fn to_pinyin<'a>(str: *const c_char) -> PinyinStrIter<'a> {
    let text = unsafe { CStr::from_ptr(str) }.to_str().unwrap();
    text.to_pinyin()
}

fn to_pinyin_multi<'a>(str: *const c_char) -> PinyinMultiStrIter<'a> {
    let text = unsafe { CStr::from_ptr(str) }.to_str().unwrap();
    text.to_pinyin_multi()
}

#[cfg(test)]
mod tests {
    use pinyin::{ToPinyin, ToPinyinMulti};
    use crate::to_pinyin;
    use std::os::raw::c_char;
    use std::ffi::{CString, CStr};

    #[test]
    fn it_works() {
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
        assert_eq!(2 + 2, 4);
    }
}


