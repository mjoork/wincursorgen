use wincursorgen::arguments::Arguments;

fn main() {
    let args = Arguments::parse();

    let configs = wincursorgen::cursor_config_parsing::parse_config_file(&args.config);

    for config in configs {
        println!(
            "size: {}\nx: {}\ny: {}\nimage: {}\n",
            config.cursor_size, config.cursor_offset_x, config.cursor_offset_y, config.cursor_image
        )
    }
}
