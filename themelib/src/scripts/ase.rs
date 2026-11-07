use std::{
    borrow::Cow,
    fs::File,
    io::{Result as IoResult, Write},
    path::{Path, PathBuf},
};

use anyhow::{Result as AnyResult, bail};
use byteorder::{BigEndian, WriteBytesExt};
use glam::prelude::*;

use crate::{
    template::fmt_string,
    theme::{ColorRef, Theme},
};

pub fn make_ase(theme: &Theme, output: Option<&Path>, force: bool) -> AnyResult<()> {
    let output = if let Some(root) = output {
        if root.is_dir() {
            let mut path = root.join(fmt_string(&theme.meta.name, "k"));
            path.add_extension("ase");
            Cow::Owned(path)
        } else {
            Cow::Borrowed(root)
        }
    } else {
        let mut path = PathBuf::from(fmt_string(&theme.meta.name, "k"));
        path.add_extension("ase");
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
    write_ase(theme, &mut w)?;
    Ok(())
}

pub fn write_ase(theme: &Theme, w: &mut impl Write) -> IoResult<()> {
    w.write_all(b"ASEF")?;
    w.write_u16::<BigEndian>(1)?; // Version major
    w.write_u16::<BigEndian>(0)?; // Version minor

    // Chunk count = group start + colors + group end
    w.write_u32::<BigEndian>(ColorRef::iter_base().count() as u32 + 2)?;

    let mut chunk = Vec::with_capacity(256);
    write_ase_name(&mut chunk, &fmt_string(&theme.meta.name, "T"))?;
    write_ase_chunk(w, 0xC001, &chunk)?;

    for i in ColorRef::iter_base() {
        let color = theme.get(i).without_a().to_srgb();
        chunk.clear();

        write_ase_name(&mut chunk, &color.to_string())?;
        chunk.write_all(b"RGB ")?;
        chunk.write_f32::<BigEndian>(color.r.saturate() as f32)?;
        chunk.write_f32::<BigEndian>(color.g.saturate() as f32)?;
        chunk.write_f32::<BigEndian>(color.b.saturate() as f32)?;
        chunk.write_u16::<BigEndian>(2)?; // Normal color type
        write_ase_chunk(w, 0x0001, &chunk)?;
    }

    // Terminate the group
    w.write_u16::<BigEndian>(0xC002)?;
    w.write_u32::<BigEndian>(0)?;

    Ok(())
}

fn write_ase_chunk(w: &mut impl Write, code: u16, chunk: &[u8]) -> IoResult<()> {
    w.write_u16::<BigEndian>(code)?;
    w.write_u32::<BigEndian>(chunk.len() as u32)?;
    w.write_all(chunk)?;
    Ok(())
}

fn write_ase_name(w: &mut impl Write, name: &str) -> IoResult<()> {
    let name = name.encode_utf16().flat_map(|u| u.to_be_bytes()).collect::<Vec<u8>>();
    w.write_u16::<BigEndian>(name.len() as u16 / 2 + 1)?;
    w.write_all(&name)?;
    w.write_u16::<BigEndian>(0)?; // Null terminator
    Ok(())
}
