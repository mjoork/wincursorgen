# wincursorgen

A program to convert series of PNGs into windows cursor files (`.cur`). For more explanation see [what](#what) and [why](#why).

## What?

This is my attempt to recreate Linux utility `xcursorgen`, which generates x-cursor files, on Windows to generate windows cursor files.

## Why?

I've found this one cursor for Linux that I desperately want to port to Windows, but I also don't want to impact the workflow of the author or writing a python script. Since this is just a pure desire of mine, It doesn't bother me if I spend a week or a month writing this software. I just hope that it will be useful.

Therefore I imagine it fitting nicely into already existing automation scripts and makefiles that cursor makers use. Instead of completely rewriting their scripts, they can just duplicate a make target and replace `xcursorgen` with `wincursorgen` and get the same output, but for windows.

## TODO
- [x] Parsing cursor config files
- [ ] Converting images to cursor files according to config

## Disclaimer

I don't concern myself with the fact that Windows' cursor themes don't allow the same amount of customization as in Linux. This is only a tool, its' purpose is clear, use it as you please.
