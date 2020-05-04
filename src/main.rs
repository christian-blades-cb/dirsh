use anyhow::{Context, Result};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(structopt::StructOpt)]
#[structopt(about)]
struct Args {
    #[structopt(parse(from_os_str), default_value = "./")]
    path: PathBuf,
    /// don't parse gitignore (including global gitignore and local git excludes)
    #[structopt(long = "no-gitignore")]
    no_gitignore: bool,
}

#[paw::main]
fn main(args: Args) -> Result<()> {
    let mut walk = ignore::WalkBuilder::new(&args.path)
        .add_custom_ignore_filename(".hashignore")
        .hidden(false)
        .ignore(false)
        .git_ignore(!args.no_gitignore)
        .git_global(!args.no_gitignore)
        .git_exclude(!args.no_gitignore)
        .require_git(false)
        .sort_by_file_path(|a, b| a.cmp(b))
        .build();

    let ctx = walk
        .try_fold(md5::Context::new(), hash_file)
        .context("Could not complete hashing")?;

    let digest = ctx.compute();
    println!("{}", data_encoding::BASE32_NOPAD.encode(&digest.0));
    Ok(())
}

fn hash_file(
    mut ctx: md5::Context,
    entry: std::result::Result<ignore::DirEntry, ignore::Error>,
) -> Result<md5::Context> {
    use std::os::unix::fs::MetadataExt;

    let entry = entry?;
    let path = entry.path();
    if !path.is_file() {
        return Ok(ctx);
    }
    let fd = File::open(path).with_context(|| format!("failed to open file {}", path.display()))?;

    let md = fd.metadata()?;
    let metadata = MetaData {
        mtime: md.mtime(),
        mode: md.mode(),
    };

    unsafe { ctx.consume(any_as_u8_slice(&metadata)) };
    let mut buf = BufReader::new(fd);
    loop {
        let chunk = buf.fill_buf().context("Failure while reading file")?;
        let len = chunk.len();
        if len == 0 {
            break;
        }
        ctx.consume(chunk);
        buf.consume(len);
    }
    Ok(ctx)
}

#[repr(packed)]
#[allow(dead_code)]
struct MetaData {
    mtime: i64,
    mode: u32,
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
}
