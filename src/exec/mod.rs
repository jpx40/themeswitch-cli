use crate::parser::{Exec, Script};
use camino::Utf8Path;
use std::fs::File;
use std::os::unix::fs::PermissionsExt;
pub enum Executable {
    Script(Script),
    Shell(Exec),
    Command(String),
    Exec(Exec),
    None,
}

type E = Executable;

pub fn exec(exec: &E) -> Result<(), String> {
    match exec {
        E::Script(script) => {
            let path = script.path.clone();

            if path.is_some() {
                let path = path.unwrap();
                if is_executable(&path).unwrap_or(false) {
                    let mut cmd = std::process::Command::new(&path);
                    if script.arg.is_some() {
                        let arg = script.arg.clone().unwrap();
                        for a in arg.iter() {
                            cmd.arg(a);
                        }
                    }
                    cmd.stdout(std::process::Stdio::inherit());
                    cmd.stderr(std::process::Stdio::inherit());
                    cmd.stdin(std::process::Stdio::inherit());

                    match cmd.spawn() {
                        Ok(mut cmd) => {
                            if let Err(e) = cmd.wait() {
                                return Err(format!("Could not execute command: {e}"));
                            }
                        }
                        Err(e) => {
                            return Err(format!("Could not execute command: {e}"));
                        }
                    }
                } else {
                    return Err(format!(
                        "Could not execute command: {path} , path is not a executable"
                    ));
                }
            }
        }
        E::Exec(exec) => {
            let path = exec.path.clone();

            if path.is_some() {
                let path = path.unwrap();
                if is_executable(&path).unwrap_or(false) {
                    let mut cmd = std::process::Command::new(&path);
                    if exec.arg.is_some() {
                        let arg = exec.arg.clone().unwrap();
                        for a in arg.iter() {
                            cmd.arg(a);
                        }
                    }
                    cmd.stdout(std::process::Stdio::inherit());
                    cmd.stderr(std::process::Stdio::inherit());
                    cmd.stdin(std::process::Stdio::inherit());

                    match cmd.spawn() {
                        Ok(mut cmd) => {
                            if let Err(e) = cmd.wait() {
                                return Err(format!("Could not execute command: {e}"));
                            }
                        }
                        Err(e) => {
                            return Err(format!("Could not execute command: {e}"));
                        }
                    }
                } else {
                    return Err(format!(
                        "Could not execute command: {path} , path is not a executable"
                    ));
                }
            } else {
                if let Some(cmd) = &exec.cmd {
                    let command = &exec.cmd.clone().unwrap();

                    let mut cmd = std::process::Command::new(&command.clone());
                    if exec.arg.is_some() {
                        let arg = exec.arg.clone().unwrap();
                        for a in arg.iter() {
                            cmd.arg(a);
                        }
                    }
                    cmd.stdout(std::process::Stdio::inherit());
                    cmd.stderr(std::process::Stdio::inherit());
                    cmd.stdin(std::process::Stdio::inherit());

                    match cmd.spawn() {
                        Ok(mut cmd) => {
                            if let Err(e) = cmd.wait() {
                                return Err(format!("Could not execute command: {e}"));
                            }
                        }
                        Err(e) => {
                            return Err(format!("Could not execute command: {e}"));
                        }
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}
enum Check {
    Path(String),
    Exec(String),
}
pub fn is_executable(path: &str) -> std::io::Result<bool> {
    let path = Utf8Path::new(path);
    let file = File::open(path)?;

    let perms = file.metadata()?.permissions().mode();
    let mut digit: [u32; 3] = [0; 3];

    for (i, c) in perms.to_string().chars().enumerate() {
        if i > 2 {
            break;
        }
        let d: u32 = c.to_string().parse().unwrap_or(0);
        digit[i] = d;
    }

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
    match digit[1] {
        1 => {
            one = true;
        }
        5 => one = true,
        7 => one = true,
        _ => {}
    }
    match digit[2] {
        1 => two = true,
        5 => two = true,
        7 => two = true,
        _ => {}
    }
    if one || two || zero {
        Ok(true)
    } else {
        Ok(false)
    }
}
