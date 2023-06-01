use std::{fs, fs::{File, OpenOptions}, 
          io, io::prelude::*,
          os::unix,
          path::Path,
          fmt::{Binary, Formatter}
         };

struct PpmEncoder {
    w: u16,
    h: u16,
}

impl PpmEncoder {
    fn new(width: u16, height: u16) -> PpmEncoder {
        PpmEncoder { w: width, h: height }
    }

    fn write_header(self, path: &Path) -> io::Result<()> {
        let mut f: File = File::create(path)?;

        f.write_fmt(format_args!("P6\n{}\n{}", self.w, self.h))
    }

    fn write_image_data(self, b: &[u8], path: &Path) -> io::Result<()> {
        let mut f = OpenOptions::new()
                            .append(true)
                            .write(true)
                            .open(path)?;
        
        f.write_all(b)
    }
}

fn main() {
    println!("Hello, world?");  
}
