# Quickly and flexibly generate content based on handlebars templates

## Legal

Dual-licensed under `MIT` or the [UNLICENSE](http://unlicense.org/).

## Installation

    cargo install handlebars-magic

## Usage

```
Generates documentation from handlebars templates

Usage: handlebars-magic <INPUT> <OUTPUT>

Arguments:
  <INPUT>   The input folder with templates
  <OUTPUT>  The output folder

Options:
  -h, --help     Print help
  -V, --version  Print version

```

For each file in the input folder, a corresponding file will be created in the output folder.

For example, this project generates updated documentation using the following call:

```
handlebars-magic templates .
```

## Supported helpers

### `from`

Searches for the prefix and starts with it if it is found. Otherwise, the entire string is returned.

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

Allows to insert markdown's fenced code block. Content would be trimmed.

    {{ codeblock "bash" "echo test" }}

renders to:

    ```bash
    echo test
    ```

### `exec`

Allows to include output of command.

    {{ exec "echo test" }}

renders to:

    test

### `handlebars_misc_helpers`

All helpers from [handlebars_misc_helpers](https://crates.io/crates/handlebars_misc_helpers).