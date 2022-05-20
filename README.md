# scoop-hook

Hook your scoop sub-commands in your `$env:PATH`

Inspired by [`scoop-search`](https://github.com/shilangyu/scoop-search)

[Scoop](https://scoop.sh/) sub-commands can be extended by scripts named as `scoop-<sub-command>.ps1`. By default, `scoop.ps1` only searches powershell scripts in `libexec` and `shims` folders. 

A hook provided by `scoop-search` can make `scoop search` sub-command to call `scoop-search.exe` binary. However, it will not work if you get some other sub-commands in the future.

This tool can dynamicly find all executables named as `scoop-<sub-command>.exe` in folders listed in the environment variable `$env:PATH`, and execute them as a sub-command of scoop.

Please open an issue or pull request if it does not work for you.

## Installation

Add [siku](https://github.com/amorphobia/siku) bucket and then install with scoop.

```PowerShell
scoop bucket add siku https://github.com/amorphobia/siku
scoop install scoop-hook
```

## Usage

Invoke the hook from PowerShell or add it to your `$PROFILE`

```PowerShell
Invoke-Expression (&scoop-hook --hook)
```

Then use any sub-commands by `scoop <sub-command>`.

## License

[AGPL 3.0](LICENSE) or later.
