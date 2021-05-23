# Generates documentation from handlebars templates

## Legal

Dual-licensed under `MIT` or the [UNLICENSE](http://unlicense.org/).

## Installation

    cargo install handlebars-magic

## Usage

    handlebars-magic 0.3.0
    Generates documentation from handlebars templates

    USAGE:
        handlebars-magic <input> <output>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    ARGS:
        <input>     The input folder with templates
        <output>    The output folder

## Supported helpers

### `from`

Searches for the prefix and starts with it if found. Else returns the whole string.

    {{ from "begin" "prefix begin text end" }}

renders to:

    begin text end

### `render`

Processes an argument as `handlebars`'s template.

    {{ render "some handlebars template" }}

renders to:

    some handlebars template

This does not look useful until we use it in conjuction with other helper such as `read_to_str` from [handlebars_misc_helpers](https://crates.io/crates/handlebars_misc_helpers):

    {{ render ( read_to_str "templates/README.md" ) }}

### `codeblock`

Allows to insert markdown's fanced code block. Content would be trimmed.

    {{ codeblock "bash" "echo test" }}

renders to:

    ```bash
    echo test
    ```

### `handlebars_misc_helpers`

All helpers from [handlebars_misc_helpers](https://crates.io/crates/handlebars_misc_helpers).