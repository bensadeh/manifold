<p align="center">
  <img src="assets/manifold.png" width="230"/>
</p>

#                                                                                                                                                                                                                                                                                                                                    

<p align="center">
A general purpose highlighting library 
</p>

## Overview

`manifold` is a general-purpose highlighting engine for highlighting common objects such as dates, numbers, keywords,
and UUIDs. It works by surrounding these items with ANSI color codes, enabling them to be displayed in color in the
terminal.

If you're just interested in a cli for highlighting text, see [`tailspin`](https://github.com/bensadeh/tailspin).

## Supported Highlight Groups

- Custom keywords
- Numbers
- UUIDs
- URLs
- IPs (v4 & v6)
- Dates
- Quotes
- Unix File Paths
- Key Value pairs
- Pointer Addresses
- Unix Processes

## Example

```rust
use inlet_manifold::*;

fn main() {
    let highlighter = Highlighter::default();

    let input = "Hello 42 world".to_string();
    let output = highlighter.apply(input);

    println!("{}", input);  // "Hello 42 world"
    println!("{}", output); // "Hello \u{1b}[36m42\u{1b}[0m world!"
}
```