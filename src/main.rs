/*!
# Generates documentation from handlebars templates

## Legal

Dual-licensed under `MIT` or the [UNLICENSE](http://unlicense.org/).

## Installation

    cargo install handlebars-magic

## Usage

    handlebars-magic 0.1.0
    Generates documentation from handlebars templates

    USAGE:
        handlebars-magic <input> <output>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    ARGS:
        <input>     The input folder with templates
        <output>    The output folder
*/
mod cli;

use std::{
    collections::VecDeque,
    fs::{self, File, OpenOptions},
};

use anyhow::anyhow;
use cli::Cli;
use handlebars::{handlebars_helper, Handlebars};
use handlebars::{Context, Helper, JsonRender, Output, RenderContext, RenderError};
use log::info;
use structopt::StructOpt;

fn render(
    h: &Helper,
    hbs: &Handlebars,
    context: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for format helper."))?;
    let rendered = hbs
        .render_template(param.value().render().as_str(), &context.data())
        .map_err(|_err| RenderError::new("Cannot render template"))?;
    out.write(rendered.as_ref())?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let cli = Cli::from_args();

    if !cli.input.is_dir() {
        return Err(anyhow!(
            "Input must be an existing directory: {}",
            cli.input.to_string_lossy()
        ));
    }

    fs::create_dir_all(&cli.output)?;

    let mut dirs = VecDeque::new();
    dirs.push_back(cli.input.clone());

    let mut handlebars = handlebars_misc_helpers::new_hbs();

    handlebars_helper!(from: |f: str, c: str| {
        if let Some(pos) = c.find(f) {
            &c[pos..]
        } else {
            c
        }
    });
    handlebars.register_helper("from", Box::new(from));

    handlebars.register_helper("render", Box::new(render));

    while !dirs.is_empty() {
        let dir = dirs.pop_front().unwrap();
        for entry in dir.read_dir()? {
            if let Ok(entry) = entry {
                let path = entry.path();
                let suffix = path.strip_prefix(&cli.input)?;
                let target = cli.output.join(suffix);
                if path.is_dir() {
                    dirs.push_back(path);
                    fs::create_dir_all(target)?;
                } else {
                    info!("{:?} -> {:?}", path, target);
                    let mut input = File::open(path)?;
                    let output = OpenOptions::new()
                        .create(true)
                        .write(true)
                        .truncate(true)
                        .open(target)?;
                    handlebars.render_template_source_to_write(&mut input, &(), output)?;
                }
            }
        }
    }

    Ok(())
}
