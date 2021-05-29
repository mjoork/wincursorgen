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
			<th width="300px">Script</th>
			<th>Purpose</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td width="300px"><code>create_static_cursor.(sh|ps1)</code></td>
			<td>Creates a static windows cursor from the <code>./tests/images/32-test-cursor.png</code> image using <code>wincursorgen</code> according to the <code>./tests/configs/test-cursor.in</code> config file.</td>
		</tr>
	</tbody>
</table>

### Testing with `cargo test`

Tests in `./tests/` directory are simple shell scripts to test an already compiled program. However there are some doc-tests available.

### Cursor images for testing
Created by me, anyone can use them.

- Static: <img align="middle" src="./tests/images/32-test-cursor.png" alt="32 by 32 pixels PNG cursor for testing" title="32 by 32 pixels PNG cursor for testing" />
- Animated: *none yet*


## Things this program can do
- [x] Convert images to static cursor according to config
- [ ] Convert images to animated cursor files according to config

## What?

In Linux world there's a utility called `xcursorgen`. It is used to *"create an X cursor file from a collection of PNG images"* or just to create cursors from PNGs basically.

This program - `wincursorgen` - is meant to act as twin of `xcursorgen` for Windows CUR files, to *"create a Windows cursor file from a collection of PNG images"*, that is.

## Why?

I've found this one cursor for Linux that I desperately want to port to Windows, but I also don't want to impact the workflow of the author or writing a python script. Since this is just a pure desire of mine, It doesn't bother me if I spend a week or a month writing this software. I just hope that it will be useful.

Therefore I imagine it fitting nicely into already existing automation scripts and makefiles that cursor makers use. Instead of completely rewriting their scripts, they can just duplicate a make target and replace `xcursorgen` with `wincursorgen` and get the same output, but for Windows.

## Does it work?

Every stable release is tested to work, but if you find a bug, open an issue.
