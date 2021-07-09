## Path Compactor

Simple tool for squishing long file system paths into shorter abbreviations. Kind of nice for shell prompts, not so much for anything else.

Iterates over path fragments between directory separators and squashes them based on length and case formatting. Home directory prefixes are replaced with with a `~` character.

| Full | Compact |
| --- | --- |
| `/path/to/long-directory-name` | `/path/to/ldn` |
| `/Users/someone/projects/masterpiece` | `~/pro/mas` |
| `/camelCaseFragment` | `/cCF` |
| `/snake_case_v2` | `/scv2` |

Invoking the executable will compact the first provided argument or else the current working directory.

### Shell prompt

`cargo build` the thing and move the binary somewhere easy to remember.

Example with bash, assuming the binary is located at `~/bin/pc`:

```sh
# ~/.bashrc
PS1="\$(dirname \$(~/bin/pc))/\$(basename \$PWD) $ "
```

Or PowerShell:

```powershell
function Prompt {
  $P = (~/bin/pc.exe)
  Write-Host "$(Split-Path $P)\" -ForegroundColor darkgray -NoNewline
  Write-Host (Split-Path $P -Leaf) -NoNewline
  Write-Host " PS>" -ForegroundColor cyan -NoNewline
  return " "
}
```
