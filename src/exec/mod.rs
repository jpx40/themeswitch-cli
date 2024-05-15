use crate::parser::Script;

use camino::Utf8Path;
use std::fs::File;
use std::os::unix::fs::PermissionsExt;

use std::ffi::CString;

use libc::{c_char, c_int, faccessat, AT_FDCWD, F_OK, R_OK, W_OK, X_OK};

pub fn is_executable(path: &str) -> std::io::Result<bool> {
    let path = Utf8Path::new(path);
    let file = File::open(path)?;

    let perms = {
        let mut out = false;
        let perms = file.metadata()?.permissions().mode();
        let mut digit: [u32; 3] = [0; 3];

        for (i, c) in perms.to_string().chars().enumerate() {
            if i > 2 {
                break;
            }
            let d: u32 = c.to_string().parse().unwrap();
            digit[i] = d;
        }
        println!("{:?}", path);
        println!("{:?}", digit);

        let mut zero = false;
        let mut one = false;
        let mut two = false;
        match digit[0] {
            1 => {
                zero = true;
            }
            5 => {
                zero = true;
            }
            7 => {
                zero = true;
            }
            _ => {}
        }
        if one || two || zero {
            true
        } else {
            false
        }
    };

    Ok(perms)
}
