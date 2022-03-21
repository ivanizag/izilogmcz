use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom, Result, Error, ErrorKind};

/*

See: https://rio.early8bitz.de/rio/rio-fs-sector.htm

RIO disk format:
 - single sided
 - 77 tracks
 - 32 sectors per track

 Each sector has 
 - 1 byte for the sector number
 - 1 byte for the track number
 - 128 bytes for the data
 - 2 bytes for the sector and track of the preceding data in the file
 - 2 bytes for the sector and track of the following data in the file
 - 2 bytes for the CRC

*/

pub const SECTOR_SIZE: usize = 128;
pub const SECTOR_SIZE_IN_FILE: usize = SECTOR_SIZE + 8;
pub const SECTOR_COUNT: usize = 32;
const TRACK_COUNT: usize = 77;

pub struct Media {
    pub file: Option<File>,
    pub content: Vec<u8>,
}

impl Media {
    pub fn new_from_bytes(content: &[u8]) -> Media {
        Media {
            file: None,
            content: content.to_vec(),
        }
    }

    pub fn new_from_file(filename: &str) -> Result<Media>{
        // Try opening writable, then read only
        let (mut file, readonly) = match OpenOptions::new()
            .read(true)
            .write(true)
            .open(filename)
            {
                Ok(file) => (file, false),
                _ => {
                    // Try opening read-only
                    match OpenOptions::new()
                        .read(true)
                        .open(filename)
                        {
                            Ok(file) => (file, true),
                            Err(err) => {
                                return Err(err);
                            }
                        }
                }
            };

        // Load content
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;

        // Store the file descriptor for writable files
        let file = if readonly {
            None
        } else {
            Some(file)
        };

        if content.len() != TRACK_COUNT * SECTOR_COUNT * SECTOR_SIZE_IN_FILE {
            return Err(Error::new(ErrorKind::Other, format!("Unrecognized disk image format (len {})", content.len())));
        }

        Ok(Media {
            file: file,
            content: content,
        })
    }

    pub fn read_sector(&self, track: usize, sector: usize) -> &[u8] {
        if track >= TRACK_COUNT || sector >= SECTOR_COUNT {
            return &[];
        }
        let start = (track as usize * SECTOR_COUNT + sector as usize) * SECTOR_SIZE_IN_FILE;

        // Asserts:
        /*
        let sector_in_file = self.content[start] & 0x7f;
        if sector != sector_in_file as usize {
            panic!("Sector {} in file has sector {}", sector, sector_in_file);
        }
        let track_in_file = self.content[start + 1];
        if track != track_in_file as usize {
            panic!("Track {} in file has track {}", track, track_in_file);
        }
        */

        self.content.get(start..start + SECTOR_SIZE_IN_FILE).unwrap()
    }

    pub fn write_sector(&mut self, track: usize, sector: usize, data: &[u8]) -> Result<()> {
        if track >= TRACK_COUNT || sector >= SECTOR_COUNT {
            return Err(Error::new(ErrorKind::Other, format!("Invalid track/sector {}/{}", track, sector)));
        }
        let start = (track as usize * SECTOR_COUNT + sector as usize) * SECTOR_SIZE_IN_FILE;
        let data_in_file = &mut self.content[start..start + SECTOR_SIZE_IN_FILE];
        data_in_file.copy_from_slice(data);

        if let Some(ref mut file) = self.file {
            file.seek(SeekFrom::Start(start as u64))?;
            file.write_all(&self.content[start..start + SECTOR_SIZE_IN_FILE])?;
        }

        Ok(())
    }
}