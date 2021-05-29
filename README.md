# wincursorgen

A program to convert series of PNGs into CUR cursor files (`.cur`). For more explanation see [what](#what), [why](#why) and [does it work](#does-it-work).

## Testing

There are testing scripts available with resources needed for testing in `./tests/` directory. All tests should be run from the root. Like this:

```shell
./tests/script_X.(sh|ps1)
```

List of testing scripts:

<table>
	<thead>
		<tr>
			<th width="200px">Script</th>
			<th>Purpose</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td width="200px"><code>create_static_cursor.(sh|ps1)</code></td>
			<td>Creates a static windows cursor using a PNG image according config file.</td>
		</tr>
	</tbody>
</table>

## TODO
- [x] Parsing cursor config files
- [x] Converting images to cursor files according to config
- [ ] Support animation

## Disclaimer

I don't concern myself with the fact that Windows' cursor themes don't allow the same amount of customization as in Linux. This is only a tool, its' purpose is clear, use it as you please.


## What?

In Linux world there's a utility called `xcursorgen`. It is used to *"create an X cursor file from a collection of PNG images"* or just to create cursors from pngs basically.

This program - `wincursorgen` - is meant to act as twin of `xcursorgen` for Windows CUR files, to *"create a CUR cursor file from a collection of PNG images"*, that is.

## Why?

I've found this one cursor for Linux that I desperately want to port to Windows, but I also don't want to impact the workflow of the author or writing a python script. Since this is just a pure desire of mine, It doesn't bother me if I spend a week or a month writing this software. I just hope that it will be useful.

Therefore I imagine it fitting nicely into already existing automation scripts and makefiles that cursor makers use. Instead of completely rewriting their scripts, they can just duplicate a make target and replace `xcursorgen` with `wincursorgen` and get the same output, but for Windows.

## Does it work?

Every stable release is tested to work, but if you find a bug, open an issue.