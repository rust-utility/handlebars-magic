#[doc = include_str!("../README.md")]

fn main() -> anyhow::Result<()> {
    handlebars_magic::process()
}
