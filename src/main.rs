use std::path::{Path, PathBuf};

use clap::Parser;
use image::ImageFormat;

// define the command line arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // shape options: square, vertical, landscape
    // square can be set with vertical or landscape together
    // vertical and landscape can't be set together
    #[arg(short, long, help = "Select square images")]
    square: bool,
    #[arg(short, long, help = "Select vertical images")]
    vertical: bool,
    #[arg(
        short,
        long,
        conflicts_with = "vertical",
        help = "Select landscape images"
    )]
    landscape: bool,

    // dimension options
    #[arg(
        long = "min-width",
        help = "Select images with width greater than or equal to value"
    )]
    min_width: Option<u32>,
    #[arg(
        long = "min-height",
        help = "Select images with height greater than or equal to value"
    )]
    min_height: Option<u32>,
    #[arg(long = "max-width", help = "Select images with width less than value")]
    max_width: Option<u32>,
    #[arg(
        long = "max-height",
        help = "Select images with height less than value"
    )]
    max_height: Option<u32>,

    // action
    #[arg(
        short,
        long = "move",
        help = "Move the selected images to the given directory"
    )]
    move_to: Option<String>,
    #[arg(
        short,
        long = "copy",
        conflicts_with = "move_to",
        help = "Copy the selected images to the given directory"
    )]
    copy_to: Option<String>,
    #[arg(
        short,
        long = "delete",
        conflicts_with = "move_to",
        conflicts_with = "copy_to",
        help = "Delete the selected images"
    )]
    delete: bool,

    // original image directory
    path: Option<String>,
}

// function to get the file entries from a directory
fn get_file_paths(path: &str) -> Vec<PathBuf> {
    let entries = std::fs::read_dir(path).unwrap();
    let mut file_entries = vec![];
    for entry in entries {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            file_entries.push(entry.path());
        }
    }
    file_entries
}

// function to get image dimension if given file entry can be identified by image package
fn get_image_dimension(path: &Path) -> Option<(u32, u32)> {
    let img = image::open(path);
    if let Ok(img) = img {
        return Some((img.width(), img.height()));
    }
    None
}

fn check_shape(square: bool, vertical: bool, landscape: bool, width: u32, height: u32) -> bool {
    // make sure vertical and landscape can't be set together
    if vertical && landscape {
        panic!("vertical and landscape can't be set together")
    }
    // make sure width and height are not 0
    if width == 0 || height == 0 {
        panic!("width and height can't be 0")
    }

    let is_square = width == height;
    let is_vertical = width < height;
    let is_landscape = width > height;
    if square {
        return is_square;
    }
    if vertical {
        return is_vertical;
    }
    if landscape {
        return is_landscape;
    }
    false
}

fn check_dimension(
    min_width: Option<u32>,
    min_height: Option<u32>,
    max_width: Option<u32>,
    max_height: Option<u32>,
    width: u32,
    height: u32,
) -> bool {
    if let Some(min_width) = min_width {
        if width < min_width {
            return false;
        }
    }
    if let Some(min_height) = min_height {
        if height < min_height {
            return false;
        }
    }
    if let Some(max_width) = max_width {
        if width >= max_width {
            return false;
        }
    }
    if let Some(max_height) = max_height {
        if height >= max_height {
            return false;
        }
    }
    true
}

fn take_action(action: &str, path: &PathBuf, destination_folder: Option<&PathBuf>) {
    match action {
        "move" => {
            if let Some(destination_folder) = destination_folder {
                let destionation = destination_folder.join(path.file_name().unwrap());
                if destionation.exists() {
                    println!("destination file {:?} already exists", destionation);
                }
                // create parent folder if not exists
                if !destionation.parent().unwrap().exists() {
                    std::fs::create_dir_all(destionation.parent().unwrap()).unwrap();
                }
                std::fs::rename(path, destination_folder.join(path.file_name().unwrap())).unwrap();
            } else {
                panic!("destination folder is not set")
            }
        }
        "copy" => {
            if let Some(destination_folder) = destination_folder {
                let destionation = destination_folder.join(path.file_name().unwrap());
                if destionation.exists() {
                    println!("destination file {:?} already exists", destionation);
                }
                // create parent folder if not exists
                if !destionation.parent().unwrap().exists() {
                    std::fs::create_dir_all(destionation.parent().unwrap()).unwrap();
                }
                std::fs::copy(path, destination_folder.join(path.file_name().unwrap())).unwrap();
            } else {
                panic!("destination folder is not set")
            }
        }
        "delete" => {
            std::fs::remove_file(path).unwrap();
        }
        _ => {}
    }
}

fn main() {
    let args = Args::parse();
    let source_path = args.path.as_ref().unwrap();
    let paths = get_file_paths(source_path);
    for path in paths {
        // test if the file entry can be identified by image from_extension with its extension
        let mut extension_check: bool = false;
        if let Some(extension) = path.extension() {
            if let Some(extension) = extension.to_str() {
                if ImageFormat::from_extension(extension).is_some() {
                    extension_check = true;
                }
            }
        }
        if !extension_check {
            continue;
        }
        // get the dimension of the image
        if let Some((width, height)) = get_image_dimension(&path) {
            let shape_check: bool =
                check_shape(args.square, args.vertical, args.landscape, width, height);
            let dimension_check: bool = check_dimension(
                args.min_width,
                args.min_height,
                args.max_width,
                args.max_height,
                width,
                height,
            );
            if shape_check && dimension_check {
                if args.move_to.is_some() {
                    take_action(
                        "move",
                        &path,
                        Some(&PathBuf::from(args.move_to.as_ref().unwrap())),
                    );
                    println!(
                        "{:?} ({}x{}) is moved to {:?}",
                        path,
                        width,
                        height,
                        args.move_to.as_ref().unwrap()
                    );
                } else if args.copy_to.is_some() {
                    take_action(
                        "copy",
                        &path,
                        Some(&PathBuf::from(args.copy_to.as_ref().unwrap())),
                    );
                    println!(
                        "{:?} ({}x{}) is copied to {:?}",
                        path,
                        width,
                        height,
                        args.copy_to.as_ref().unwrap()
                    );
                } else if args.delete {
                    take_action("delete", &path, None);
                    println!("{:?} ({}x{}) is deleted", path, width, height);
                }
            }
        } else {
            println!("{:?} can't be identified as an image", path);
        }
    }
}

#[cfg(test)]
mod tests;
