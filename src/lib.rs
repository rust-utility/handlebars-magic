mod cli;

use std::{
    collections::VecDeque,
    fs::{self, OpenOptions},
    process::Command,
};

use anyhow::{anyhow, Result};
use clap::Parser;
use handlebars::{
    handlebars_helper, Context, Handlebars, Helper, HelperResult, JsonRender, Output,
    RenderContext, RenderErrorReason,
};
use log::info;

use cli::Cli;

fn render(
    h: &Helper,
    hbs: &Handlebars,
    context: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).ok_or_else(|| {
        RenderErrorReason::ParamNotFoundForIndex("Param 0 is required for format helper.", 0)
    })?;
    let rendered = hbs
        .render_template(param.value().render().as_str(), &context.data())
        .map_err(|_err| RenderErrorReason::Other("Cannot render template".into()))?;
    out.write(rendered.as_ref())?;
    Ok(())
}

fn exec(
    h: &Helper,
    _hbs: &Handlebars,
    _context: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let exe = h
        .param(0)
        .ok_or_else(|| {
            RenderErrorReason::ParamNotFoundForIndex("Param 0 is required for format helper.", 0)
        })?
        .value()
        .render();

    let cmd: Vec<&str> = exe.split(' ').collect();

    if let Some((exe, args)) = cmd.split_first() {
        let output = Command::new(exe).args(args).output()?;
        if output.status.success() {
            out.write(&String::from_utf8_lossy(&output.stdout))
                .map_err(RenderErrorReason::from)
        } else {
            Err(RenderErrorReason::Other(format!(
                "Cannot run '{}': {}",
                exe,
                String::from_utf8_lossy(&output.stderr)
            )))
        }
    } else {
        Err(RenderErrorReason::Other("Cannot render template".into()))
    }
    .map_err(From::from)
}

pub fn process() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

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
    handlebars_helper!(to: |f: str, c: str| {
        if let Some(pos) = c.find(f) {
            &c[..pos]
        } else {
            c
        }
    });
    handlebars.register_helper("to", Box::new(to));

    handlebars.register_helper("render", Box::new(render));

    handlebars_helper!(codeblock: |codeblock_type: str, block: str| {
        format!("```{}\n{}\n```", codeblock_type, block.trim())
    });
    handlebars.register_helper("codeblock", Box::new(codeblock));

    handlebars.register_helper("exec", Box::new(exec));

    while !dirs.is_empty() {
        let dir = dirs.pop_front().unwrap();
        for entry in dir.read_dir()?.flatten() {
            let path = entry.path();
            let suffix = path.strip_prefix(&cli.input)?;
            let target = cli.output.join(suffix);
            if path.is_dir() {
                dirs.push_back(path);
                fs::create_dir_all(target)?;
            } else {
                info!("{:?} -> {:?}", path, target);
                let output = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(target)?;
                handlebars.render_template_to_write(&fs::read_to_string(path)?, &(), output)?;
            }
        }
    }

    Ok(())
}
