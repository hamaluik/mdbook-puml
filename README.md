# mdbook-puml

[![Crates.io](https://img.shields.io/crates/v/mdbook-puml.svg)](https://crates.io/crates/mdbook-puml)

This is a simple [mdbook](https://crates.io/crates/mdbook) preprocessor designed to replace code blocks that look like:

    ```plantuml
    Bob -> Alice : Hello!
    ```

with an inline SVG rendered using [PlantUML](https://plantuml.com/):

<figure><svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" contentscripttype="application/ecmascript" contentstyletype="text/css" height="127px" preserveAspectRatio="none" style="width:121px;height:127px;" version="1.1" viewBox="0 0 121 127" width="121px" zoomAndPan="magnify"><defs><filter height="300%" id="fl1wk7hnho7ch" width="300%" x="-1" y="-1"><feGaussianBlur result="blurOut" stdDeviation="2.0"></feGaussianBlur><feColorMatrix in="blurOut" result="blurOut2" type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 .4 0"></feColorMatrix><feOffset dx="4.0" dy="4.0" in="blurOut2" result="blurOut3"></feOffset><feBlend in="SourceGraphic" in2="blurOut3" mode="normal"></feBlend></filter></defs><g><line style="stroke: #A80036; stroke-width: 1.0; stroke-dasharray: 5.0,5.0;" x1="31" x2="31" y1="38.2969" y2="87.4297"></line><line style="stroke: #A80036; stroke-width: 1.0; stroke-dasharray: 5.0,5.0;" x1="89" x2="89" y1="38.2969" y2="87.4297"></line><rect fill="#FEFECE" filter="url(#fl1wk7hnho7ch)" height="30.2969" style="stroke: #A80036; stroke-width: 1.5;" width="42" x="8" y="3"></rect><text fill="#000000" font-family="sans-serif" font-size="14" lengthAdjust="spacingAndGlyphs" textLength="28" x="15" y="22.9951">Bob</text><rect fill="#FEFECE" filter="url(#fl1wk7hnho7ch)" height="30.2969" style="stroke: #A80036; stroke-width: 1.5;" width="42" x="8" y="86.4297"></rect><text fill="#000000" font-family="sans-serif" font-size="14" lengthAdjust="spacingAndGlyphs" textLength="28" x="15" y="106.4248">Bob</text><rect fill="#FEFECE" filter="url(#fl1wk7hnho7ch)" height="30.2969" style="stroke: #A80036; stroke-width: 1.5;" width="46" x="64" y="3"></rect><text fill="#000000" font-family="sans-serif" font-size="14" lengthAdjust="spacingAndGlyphs" textLength="32" x="71" y="22.9951">Alice</text><rect fill="#FEFECE" filter="url(#fl1wk7hnho7ch)" height="30.2969" style="stroke: #A80036; stroke-width: 1.5;" width="46" x="64" y="86.4297"></rect><text fill="#000000" font-family="sans-serif" font-size="14" lengthAdjust="spacingAndGlyphs" textLength="32" x="71" y="106.4248">Alice</text><polygon fill="#A80036" points="77,65.4297,87,69.4297,77,73.4297,81,69.4297" style="stroke: #A80036; stroke-width: 1.0;"></polygon><line style="stroke: #A80036; stroke-width: 1.0;" x1="31" x2="83" y1="69.4297" y2="69.4297"></line><text fill="#000000" font-family="sans-serif" font-size="13" lengthAdjust="spacingAndGlyphs" textLength="30" x="38" y="64.3638">hello</text></g></svg></figure>

I created this preprocessor because [mdbook-plantuml](https://crates.io/crates/mdbook-plantuml) wasn't working for me—specifically, mdbook-plantuml is currently incompatible with `mdbook watch` and `mbbook serve` because it triggers a [rebuild loop](https://github.com/sytsereitsma/mdbook-plantuml/issues/17).

This crate is quite simple and non-customizable at this point as it does all that I need it to for my own purposes. Feel free to fork and/or PR away though, and I'll be happy to include changes.

## Usage

To install _mdbook-puml_, use cargo:

```
cargo install mdbook-puml
```

Then add the following to `book.toml`:

```
[preprocessor.puml]
```

Finally, to insert a PlantUML diagram somewhere, just use a fenced code block with the language set to `plantuml`:

    ```plantuml
    Bob -> Alice : Hello!
    ```

