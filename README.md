# diskusage-rs

Basic CLI disk usage program for Linux and Windows. By default, it shows the **Mountpoint**, **Total**, **Used**, and **Free** space with no colors. You can enable color with `-c, --color`. Additionally, you can show filesystem types with `--fs`, block devices with `-d, --devices`, UUIDs with `--uuid`, and mountoptions with `-o, --mountoptions`. You can also change the style for the output table with `-s, --style`.
![Screenshot of diskusage running](https://taconator.com/static/img/projects/diskusage.avif)
*<p align="center">Screenshot generated with <a href="https://github.com/homeport/termshot">termshot</a></p>*

## Arguments
- `--fs`: Shows filesystems
- `-c, --color`: Enables color
- `--style`: Changes the style of the output table. Possible values include `ascii`, `ascii-rounded`, `blank`, `dots`, `empty`, `extended`, `markdown`, `modern` (default), `modern-rounded`, `psql`, `re-structured-text`, and `sharp`.
- `-h, --help`: Shows help
- `-v, --version`: Print version
- `-b, --bar`: Show disk usage as a bar
- `-s, --segments`: How many segments the bar should have. Only applies if `--bar` is set
### Linux specific arguments
These arguments are only available on Linux.
- `--boot`: Shows `/boot` partition
- `-d, --devices`: Shows block devices
- `--uuid`: Shows partition UUIDs
- `-o, --mountoptions`: Shows mount options

These can be combined like `-doc` and you can even do combinations like `-docsascii`.
