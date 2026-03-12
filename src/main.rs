use std::{env, io, path::PathBuf, process::Command};

fn main() -> io::Result<()> {
    let mut args_iter = env::args().skip(1).peekable();
    let sub = args_iter.peek();

    if let Some(hook) = sub {
        if hook == "--hook" || hook == "hook" {
            println!("function scoop {{ scoop-hook.exe @($args | Select-Object) }}");
            return Ok(());
        }
    }

    if let Some(sub) = sub {
        if let Some((path, exe_type)) = find_executable(&format!("scoop-{}", sub)) {
            match exe_type {
                ExecutableType::Exe => {
                    let status = Command::new(path).args(args_iter.skip(1)).status()?;
                    std::process::exit(status.code().unwrap_or(1));
                }
                ExecutableType::Ps1 => {
                    let shell = find_powershell();
                    let status = Command::new(shell)
                        .args(&[
                            "-NoProfile",
                            "-ExecutionPolicy",
                            "Unrestricted",
                            "-File",
                        ])
                        .arg(path)
                        .args(args_iter.skip(1))
                        .status()?;
                    std::process::exit(status.code().unwrap_or(1));
                }
            }
        }
    }

    let shell = find_powershell();
    let status = Command::new(shell)
        .args(&[
            "-NoProfile",
            "-ExecutionPolicy",
            "Unrestricted",
            "-Command",
            "scoop.ps1",
        ])
        .args(args_iter)
        .status()?;

    std::process::exit(status.code().unwrap_or(1));
}

fn find_executable(basename: &str) -> Option<(PathBuf, ExecutableType)> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).find_map(|dir| {
            let exe_path = dir.join(&format!("{}.exe", basename));
            if exe_path.is_file() {
                return Some((exe_path, ExecutableType::Exe));
            }
            let ps1_path = dir.join(&format!("{}.ps1", basename));
            if ps1_path.is_file() {
                return Some((ps1_path, ExecutableType::Ps1));
            }
            None
        })
    })
}

enum ExecutableType {
    Exe,
    Ps1,
}

fn find_powershell() -> PathBuf {
    find_executable("pwsh")
        .and_then(|(p, t)| match t {
            ExecutableType::Exe => Some(p),
            ExecutableType::Ps1 => None,
        })
        .unwrap_or_else(|| {
            find_executable("powershell")
                .and_then(|(p, t)| match t {
                    ExecutableType::Exe => Some(p),
                    ExecutableType::Ps1 => None,
                })
                .unwrap()
        })
}
