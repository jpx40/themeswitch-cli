use crate::parser::Script;

use camino::Utf8Path;
use std::fs::File;
use std::os::unix::fs::PermissionsExt;

use std::ffi::CString;

use libc::{c_char, c_int, faccessat, AT_FDCWD, F_OK, R_OK, W_OK, X_OK};

fn is_exuctable(path: &str) -> std::io::Result<bool> {
    let path = Utf8Path::new(path);
    let file = File::open(path)?;

    let mut perms = {
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

        for i in digit {
            if i > 5 {
                out = true;
                break;
            }
        }
        out
    };

    Ok(perms)
}
