mod whatis;

use anyhow::{anyhow, ensure, Result};
use colored::Colorize;
use serde_json::{Map, Value};
use std::{io::Write, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "whatis")]
/// A tool for telling you what a file is.
///
/// whatis is a modern replacement for the `file` command. As well as telling
/// you what a file is, it will also tell you some useful information about
/// the file.
struct Opt {
    #[structopt(long)]
    no_info: bool,

    #[structopt(long)]
    json: bool,

    #[structopt(name = "FILE")]
    path: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    ensure!(opt.path.is_file(), "argument must be a file");

    let (file_type, data) = match whatis::whatis(&opt.path) {
        Ok(None) => return Err(anyhow!("could not determine file type")),
        Err(e) => return Err(anyhow!("{}", e)),
        Ok(Some((file_type, data))) => (file_type, data),
    };

    write_result(&mut std::io::stdout(), &opt, file_type, data)
}

fn write_result<W: Write>(
    w: &mut W,
    opt: &Opt,
    file_type: whatis::FileType,
    data: Map<String, Value>,
) -> Result<()> {
    if opt.json {
        let mut value = Map::new();
        value.insert(
            "file_type".to_string(),
            Value::String(format!("{:?}", file_type)),
        );

        if !opt.no_info {
            value.insert("info".to_string(), Value::Object(data));
        }

        serde_json::to_writer_pretty(w, &value)?;
        return Ok(());
    }

    write!(w, "{}", "File type: ".bold())?;
    write!(w, "{}", format!("{:?}", file_type).green().bold())?;
    write!(w, "\n")?;

    if !data.is_empty() && !opt.no_info {
        write!(w, "{}", "Information:\n".bold())?;

        for (k, v) in data {
            write_indent(w, 2)?;
            write_key(w, &k)?;
            write_value(w, v, 2)?;
            write!(w, "\n")?;
        }
    }
    Ok(())
}

fn write_value<W: Write>(w: &mut W, value: Value, indent: usize) -> Result<()> {
    match value {
        Value::Null => write!(w, "null")?,
        Value::Bool(b) => write!(w, "{}", b)?,
        Value::Number(n) => write!(w, "{}", n)?,
        Value::String(s) => write!(w, "{}", s)?,
        Value::Array(a) => {
            for v in a {
                write!(w, "\n")?;
                write_indent(w, indent + 2)?;
                write_value(w, v, indent + 2)?;
            }
            write_indent(w, indent)?;
        }
        Value::Object(o) => {
            for (k, v) in o {
                write!(w, "\n")?;
                write_indent(w, indent + 2)?;
                write_key(w, &k)?;
                write_value(w, v, indent + 2)?;
            }
            write_indent(w, indent)?;
        }
    };

    Ok(())
}

fn write_indent<W: Write>(w: &mut W, size: usize) -> Result<()> {
    for _ in 0..size {
        write!(w, " ")?;
    }

    Ok(())
}

fn write_key<W: Write>(w: &mut W, key: &str) -> Result<()> {
    Ok(write!(w, "{}: ", key.blue())?)
}
