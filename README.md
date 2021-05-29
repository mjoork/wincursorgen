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
- Animated: <img src="./tests/images/32-test-cursor-1.png" alt="Frame #1 of animated test cursor" title="Frame #1 of animated test cursor" /><img src="./tests/images/32-test-cursor-2.png" alt="Frame #2 of animated test cursor" title="Frame #2 of animated test cursor" /><img src="./tests/images/32-test-cursor-3.png" alt="Frame #3 of animated test cursor" title="Frame #3 of animated test cursor" /><img src="./tests/images/32-test-cursor-4.png" alt="Frame #4 of animated test cursor" title="Frame #4 of animated test cursor" /><img src="./tests/images/32-test-cursor-5.png" alt="Frame #5 of animated test cursor" title="Frame #5 of animated test cursor" /><img src="./tests/images/32-test-cursor-6.png" alt="Frame #6 of animated test cursor" title="Frame #6 of animated test cursor" /><img src="./tests/images/32-test-cursor-7.png" alt="Frame #7 of animated test cursor" title="Frame #7 of animated test cursor" /><img src="./tests/images/32-test-cursor-8.png" alt="Frame #8 of animated test cursor" title="Frame #8 of animated test cursor" /><img src="./tests/images/32-test-cursor-9.png" alt="Frame #9 of animated test cursor" title="Frame #9 of animated test cursor" /><img src="./tests/images/32-test-cursor-10.png" alt="Frame #10 of animated test cursor" title="Frame #10 of animated test cursor" /><img src="./tests/images/32-test-cursor-11.png" alt="Frame #11 of animated test cursor" title="Frame #11 of animated test cursor" /><img src="./tests/images/32-test-cursor-12.png" alt="Frame #12 of animated test cursor" title="Frame #12 of animated test cursor" /><img src="./tests/images/32-test-cursor-13.png" alt="Frame #13 of animated test cursor" title="Frame #13 of animated test cursor" /><img src="./tests/images/32-test-cursor-14.png" alt="Frame #14 of animated test cursor" title="Frame #14 of animated test cursor" /><img src="./tests/images/32-test-cursor-15.png" alt="Frame #15 of animated test cursor" title="Frame #15 of animated test cursor" /><img src="./tests/images/32-test-cursor-16.png" alt="Frame #16 of animated test cursor" title="Frame #16 of animated test cursor" /><img src="./tests/images/32-test-cursor-17.png" alt="Frame #17 of animated test cursor" title="Frame #17 of animated test cursor" /><img src="./tests/images/32-test-cursor-18.png" alt="Frame #18 of animated test cursor" title="Frame #18 of animated test cursor" /><img src="./tests/images/32-test-cursor-19.png" alt="Frame #19 of animated test cursor" title="Frame #19 of animated test cursor" /><img src="./tests/images/32-test-cursor-20.png" alt="Frame #20 of animated test cursor" title="Frame #20 of animated test cursor" /><img src="./tests/images/32-test-cursor-21.png" alt="Frame #21 of animated test cursor" title="Frame #21 of animated test cursor" /><img src="./tests/images/32-test-cursor-22.png" alt="Frame #22 of animated test cursor" title="Frame #22 of animated test cursor" /><img src="./tests/images/32-test-cursor-23.png" alt="Frame #23 of animated test cursor" title="Frame #23 of animated test cursor" /><img src="./tests/images/32-test-cursor-24.png" alt="Frame #24 of animated test cursor" title="Frame #24 of animated test cursor" /><img src="./tests/images/32-test-cursor-25.png" alt="Frame #25 of animated test cursor" title="Frame #25 of animated test cursor" /><img src="./tests/images/32-test-cursor-26.png" alt="Frame #26 of animated test cursor" title="Frame #26 of animated test cursor" /><img src="./tests/images/32-test-cursor-27.png" alt="Frame #27 of animated test cursor" title="Frame #27 of animated test cursor" /><img src="./tests/images/32-test-cursor-28.png" alt="Frame #28 of animated test cursor" title="Frame #28 of animated test cursor" /><img src="./tests/images/32-test-cursor-29.png" alt="Frame #29 of animated test cursor" title="Frame #29 of animated test cursor" /><img src="./tests/images/32-test-cursor-30.png" alt="Frame #30 of animated test cursor" title="Frame #30 of animated test cursor" /><img src="./tests/images/32-test-cursor-31.png" alt="Frame #31 of animated test cursor" title="Frame #31 of animated test cursor" /><img src="./tests/images/32-test-cursor-32.png" alt="Frame #32 of animated test cursor" title="Frame #32 of animated test cursor" /><img src="./tests/images/32-test-cursor-33.png" alt="Frame #33 of animated test cursor" title="Frame #33 of animated test cursor" /><img src="./tests/images/32-test-cursor-34.png" alt="Frame #34 of animated test cursor" title="Frame #34 of animated test cursor" /><img src="./tests/images/32-test-cursor-35.png" alt="Frame #35 of animated test cursor" title="Frame #35 of animated test cursor" /><img src="./tests/images/32-test-cursor-36.png" alt="Frame #36 of animated test cursor" title="Frame #36 of animated test cursor" /><img src="./tests/images/32-test-cursor-37.png" alt="Frame #37 of animated test cursor" title="Frame #37 of animated test cursor" /><img src="./tests/images/32-test-cursor-38.png" alt="Frame #38 of animated test cursor" title="Frame #38 of animated test cursor" /><img src="./tests/images/32-test-cursor-39.png" alt="Frame #39 of animated test cursor" title="Frame #39 of animated test cursor" /><img src="./tests/images/32-test-cursor-40.png" alt="Frame #40 of animated test cursor" title="Frame #40 of animated test cursor" /><img src="./tests/images/32-test-cursor-41.png" alt="Frame #41 of animated test cursor" title="Frame #41 of animated test cursor" /><img src="./tests/images/32-test-cursor-42.png" alt="Frame #42 of animated test cursor" title="Frame #42 of animated test cursor" /><img src="./tests/images/32-test-cursor-43.png" alt="Frame #43 of animated test cursor" title="Frame #43 of animated test cursor" /><img src="./tests/images/32-test-cursor-44.png" alt="Frame #44 of animated test cursor" title="Frame #44 of animated test cursor" /><img src="./tests/images/32-test-cursor-45.png" alt="Frame #45 of animated test cursor" title="Frame #45 of animated test cursor" /><img src="./tests/images/32-test-cursor-46.png" alt="Frame #46 of animated test cursor" title="Frame #46 of animated test cursor" /><img src="./tests/images/32-test-cursor-47.png" alt="Frame #47 of animated test cursor" title="Frame #47 of animated test cursor" /><img src="./tests/images/32-test-cursor-48.png" alt="Frame #48 of animated test cursor" title="Frame #48 of animated test cursor" /><img src="./tests/images/32-test-cursor-49.png" alt="Frame #49 of animated test cursor" title="Frame #49 of animated test cursor" /><img src="./tests/images/32-test-cursor-50.png" alt="Frame #50 of animated test cursor" title="Frame #50 of animated test cursor" /><img src="./tests/images/32-test-cursor-51.png" alt="Frame #51 of animated test cursor" title="Frame #51 of animated test cursor" /><img src="./tests/images/32-test-cursor-52.png" alt="Frame #52 of animated test cursor" title="Frame #52 of animated test cursor" /><img src="./tests/images/32-test-cursor-53.png" alt="Frame #53 of animated test cursor" title="Frame #53 of animated test cursor" /><img src="./tests/images/32-test-cursor-54.png" alt="Frame #54 of animated test cursor" title="Frame #54 of animated test cursor" /><img src="./tests/images/32-test-cursor-55.png" alt="Frame #55 of animated test cursor" title="Frame #55 of animated test cursor" /><img src="./tests/images/32-test-cursor-56.png" alt="Frame #56 of animated test cursor" title="Frame #56 of animated test cursor" /><img src="./tests/images/32-test-cursor-57.png" alt="Frame #57 of animated test cursor" title="Frame #57 of animated test cursor" /><img src="./tests/images/32-test-cursor-58.png" alt="Frame #58 of animated test cursor" title="Frame #58 of animated test cursor" /><img src="./tests/images/32-test-cursor-59.png" alt="Frame #59 of animated test cursor" title="Frame #59 of animated test cursor" /><img src="./tests/images/32-test-cursor-60.png" alt="Frame #60 of animated test cursor" title="Frame #60 of animated test cursor" />


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
