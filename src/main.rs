use std::{
    io::{BufReader, BufWriter},
    process::exit,
};

use wincursorgen::{args::Arguments, processing::parse_config_file};

fn main() {
    let args = Arguments::parse();
    let cur_infos = parse_config_file(&args.config);

    let mut cursor = ico::IconDir::new(ico::ResourceType::Cursor);

    for cur_info in &cur_infos {
        let image_path = args.prefix.join(&cur_info.image);

        let reader = if let Ok(file) = std::fs::File::open(&image_path) {
            BufReader::new(file)
        } else {
            eprintln!("Could not create a buffer for reading from file {:?}", image_path);
            exit(1);
        };

        let mut icon_image = ico::IconImage::read_png(reader).expect("Expected IconImage");
        icon_image.set_cursor_hotspot(Some((cur_info.hotspot_x, cur_info.hotspot_y)));
        let icon_entry = ico::IconDirEntry::encode(&icon_image).expect("Expected IconDirEntry");
        cursor.add_entry(icon_entry);
    }

    let writer = if let Ok(file) = std::fs::File::create(&args.output) {
        BufWriter::new(file)
    } else {
        eprintln!(
            "Could not create a buffer for writing file {:?}",
            &args.output
        );
        exit(1);
    };

    match cursor.write(writer) {
        Ok(_) => {}
        Err(e) => match e.kind() {
            std::io::ErrorKind::PermissionDenied => {
                eprintln!("Lacking permissions to write {:?}", &args.output);
                exit(1);
            }

            _ => {
                eprintln!("Some error occured while writing {:?}", &args.output);
                exit(1);
            }
        },
    };
}
