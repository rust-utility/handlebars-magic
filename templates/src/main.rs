/*!
{{ replace ( render ( read_to_str "templates/README.md" ) ) "```rust" "```rust#ignore" }}
*/

fn main() -> anyhow::Result<()> {
    handlebars_magic::process()
}
