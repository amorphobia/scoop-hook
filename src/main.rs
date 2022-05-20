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
        let exe = format!("scoop-{}.exe", sub);
        if let Some(exe) = find_exe(&exe) {
            Command::new(exe).args(args_iter.skip(1)).status()?;
            return Ok(());
        }
    }

    let shell = find_exe("pwsh.exe").unwrap_or_else(|| find_exe("powershell.exe").unwrap());
    Command::new(shell)
        .args(&[
            "-NoProfile",
            "-ExecutionPolicy",
            "Unrestricted",
            "-Command",
            "scoop.ps1",
        ])
        .args(args_iter)
        .status()?;

    Ok(())
}

fn find_exe(exe: &str) -> Option<PathBuf> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).find_map(|dir| {
            let full_path = dir.join(&exe);
            if full_path.is_file() {
                Some(full_path)
            } else {
                None
            }
        })
    })
}
