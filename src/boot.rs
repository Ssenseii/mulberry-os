use std::fs;
use std::fs::File;
use std::io::Cursor;
use std::sync::Arc;
use std::io::BufReader;
use hadris_iso::read::IsoImage;
use hadris_iso::boot::options::{BootEntryOptions, BootOptions};
use hadris_iso::boot::EmulationType;
use hadris_iso::read::PathSeparator;
use hadris_iso::write::options::{BaseIsoLevel, CreationFeatures, IsoFormatOptions};
use hadris_iso::write::{File as IsoFile, InputFiles, IsoImageWriter};

pub fn create_bootable_iso() {
    
    let boot_image_bytes: Vec<u8> = fs::read("src/boot/boot.bin").expect("Error Reading the Boot Bin");

    let readme_content = b"This is a bootable ISO";

    // 1. Prepare The Files
    let files = InputFiles {
        path_separator: PathSeparator::ForwardSlash,
        files: vec![
            IsoFile::File {
                name: Arc::new("boot.bin".to_string()),
                contents: boot_image_bytes
            },
                IsoFile::File {
                name: Arc::new("README.TXT".to_string()),
                contents: readme_content.to_vec(),
            },
            IsoFile::Directory {
                name: Arc::new("docs".to_string()),
                children: vec![IsoFile::File {
                    name: Arc::new("MANUAL.TXT".to_string()),
                    contents: b"User manual goes here.\n".to_vec(),
                }],
            },
        ]
    };

    // 2. Boot Options
    let boot_options = BootOptions {
        write_boot_catalog: true,
        default: BootEntryOptions {
            boot_image_path: "boot.bin".to_string(),
            // load four sectors
            load_size: Some(std::num::NonZeroU16::new(4).unwrap()),
            // enable it in case we need it later.
            boot_info_table: false,
            grub2_boot_info: false,
            emulation: EmulationType::NoEmulation,
        },
        entries: vec![],
    };

    // 3. Create the ISO
    let format_options = IsoFormatOptions {
        volume_name: "MULBERRY_ISO".to_string(),
        system_id: None,
        volume_set_id: None,
        publisher_id: None,
        preparer_id: None,
        application_id: None,
        sector_size: 2048,
        path_separator: PathSeparator::ForwardSlash,
        features: CreationFeatures {
            filenames: BaseIsoLevel::Level1 {
                supports_lowercase: false,
                supports_rrip: false,
            },
            long_filenames: false,
            joliet: None,
            rock_ridge: None,
            el_torito: Some(boot_options),
            hybrid_boot: None,
        },
        strict_charset: false,
    };

    // 512KB buffer
    let mut buffer = Cursor::new(vec![0u8; 512 * 1024]);        
    IsoImageWriter::create(&mut buffer, files, format_options).expect("couldn't create ISO image");

    // 4. Write into File
    let iso_data = buffer.into_inner();
    std::fs::write("mulberry.iso", &iso_data).expect("Failed to write the ISO file");

    println!("Created mulberry.iso ({} bytes)", iso_data.len());
}


pub fn read_bootable_iso() {

    

    let file = File::open("mulberry.iso").expect("Couldn't Find Specified Bootable ISO");
    let reader = BufReader::new(file);
    let image = IsoImage::open(reader).expect("Couldn't read ISO File Buffer");

    let root = image.root_dir();
    for entry in root.iter(&image).entries() {
        let entry = entry.expect("RBI: Bad Entry");
        println!("File: {:?}", String::from_utf8_lossy(entry.name()));
    }
}