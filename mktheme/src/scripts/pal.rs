use std::{
    borrow::Cow,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result as AnyResult};
use byteorder::{LittleEndian, WriteBytesExt};

use crate::{
    template::fmt_string,
    theme::{Indexer, Theme},
};

pub fn make_pal(theme: &Theme, output: Option<&Path>, force: bool) -> AnyResult<()> {
    let output = if let Some(root) = output {
        if root.is_dir() {
            let mut path = root.join(fmt_string(&theme.name, "k"));
            path.add_extension("pal");
            Cow::Owned(path)
        } else {
            Cow::Borrowed(root)
        }
    } else {
        let mut path = PathBuf::from(fmt_string(&theme.name, "k"));
        path.add_extension("pal");
        Cow::Owned(path)
    };

    if output.exists() {
        if force {
            println!("Overwriting {}", output.display());
        } else {
            bail!("{}: Already exists (use --force to overwrite)", output.display());
        }
    } else {
        println!("Writing {}", output.display());
    }

    let mut w = File::create(&output)?;
    let color_count = Indexer::iter_base().count();
    let filesize = 24 + 4 * color_count;

    w.write_all(b"RIFF")?;
    w.write_u32::<LittleEndian>(filesize as u32 - 8)?;
    w.write_all(b"PAL data")?;
    w.write_u32::<LittleEndian>((filesize - 20) as u32)?;
    w.write_u16::<LittleEndian>(0x0300)?;
    w.write_u16::<LittleEndian>(color_count as u16)?;

    for i in Indexer::iter_base() {
        let [r, g, b] = theme[i].to_srgb().to_bytes();
        w.write_all(&[r, g, b, 255])?;
    }

    Ok(())
}
