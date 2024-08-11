# disk-usage-rs
Basic CLI disk usage program for Linux. By default, it shows the **Mountpoint**, **Total**, **Used**, and **Free** space with no colors. You can enable color with `-c, --color`. Additionally, you can show filesystem types with `--fs`, block devices with `-d, --devices`, UUIDs with `--uuid`, and mountoptions with `-o, --mountoptions`. You can also change the style for the output table with `-s, --style`.

## Arguments
- `--fs`: Shows filesystems
- `-d, --devices`: Shows block devices
- `--uuid`: Shows partition UUIDs
- `-o, --mountoptions`: Shows mount options
- `-c, --color`: Enables color
- `-s, style`: Changes the style of the output table. Possible values include `ascii`, `ascii-rounded`, `blank`, `dots`, `empty`, `extended`, `markdown`, `modern` (default), `modern-rounded`, `psql`, `re-structured-text`, and `sharp`.
- `-h, --help`: Shows help
- `-v, --version`: Print version

These can be combined like `-doc` and you can even do combinations like `-docsascii`.
