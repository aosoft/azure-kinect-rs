use super::bindings::*;
use super::error::Error;
use std::ptr;

pub(crate) fn get_k4a_string(
    f: &Fn(*mut ::std::os::raw::c_char, *mut usize) -> k4a_buffer_result_t,
) -> Result<String, Error> {
    unsafe {
        let mut buffer: usize = 0;
        let r = (f)(ptr::null_mut(), &mut buffer);
        match r {
            k4a_buffer_result_t::K4A_BUFFER_RESULT_SUCCEEDED => Ok(String::new()),
            k4a_buffer_result_t::K4A_BUFFER_RESULT_TOO_SMALL => {
                if (buffer > 1) {
                    let mut retstr = String::with_capacity(buffer);
                    retstr.as_mut_vec().set_len(buffer - 1);
                    Error::from((f)(
                        retstr.as_mut_ptr() as *mut ::std::os::raw::c_char,
                        &mut buffer,
                    ))
                    .to_result(retstr)
                } else {
                    Err(Error::from(r))
                }
            }
            _ => Err(Error::from(r)),
        }
    }
}

pub(crate) fn get_k4a_binary_data(
    f: &Fn(*mut u8, *mut usize) -> k4a_buffer_result_t,
) -> Result<Vec<u8>, Error> {
    unsafe {
        let mut buffer: usize = 0;
        let r = (f)(ptr::null_mut(), &mut buffer);
        match r {
            k4a_buffer_result_t::K4A_BUFFER_RESULT_SUCCEEDED => Ok(Vec::<u8>::new()),
            k4a_buffer_result_t::K4A_BUFFER_RESULT_TOO_SMALL => {
                if (buffer > 1) {
                    let mut retbuf = Vec::<u8>::with_capacity(buffer);
                    retbuf.set_len(buffer);
                    Error::from((f)(retbuf.as_mut_ptr(), &mut buffer)).to_result(retbuf)
                } else {
                    Err(Error::from(r))
                }
            }
            _ => Err(Error::from(r)),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::super::bindings::*;
    use super::super::error;
    use super::*;

    #[test]
    fn test() {
        let t1 = "abcdefg";
        let ct1 = std::ffi::CString::new(t1).unwrap();

        let f: &Fn(*mut ::std::os::raw::c_char, *mut usize) -> k4a_buffer_result_t = &|s, len| unsafe {
            *len = t1.len() + 1;
            if s == std::ptr::null_mut() {
                k4a_buffer_result_t::K4A_BUFFER_RESULT_TOO_SMALL
            } else {
                std::ptr::copy_nonoverlapping(ct1.as_ptr(), s, t1.len() + 1);
                k4a_buffer_result_t::K4A_BUFFER_RESULT_SUCCEEDED
            }
        };

        let rst1 = get_k4a_string(&f);
        let st1 = rst1.unwrap();
        assert_eq!(st1, t1)
    }
}
