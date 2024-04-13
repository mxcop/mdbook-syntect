# <sup>`MDBook`</sup> Syntect
*Disclaimer: this extension is not from the creators of syntect!*

A basic [`mdbook`](https://github.com/rust-lang/mdBook) extension for highlighting codeblocks at "compile" time.<br>
[`syntect`](https://github.com/trishume/syntect) is used to do all the syntax highlighting.

This project currently just a **proof of concept**, don't expect too much!<br>
However, feel free to contribute \:)

## Usage
<sub>book.toml</sub>
```toml
# add this line to your book.toml
[preprocessor.syntect]
custom-theme = "assets/<your-theme>.tmTheme"
```

<br>

<sub>© 2023 Max, All rights reserved.</sub>
