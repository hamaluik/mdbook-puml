# mdbook-puml

This is a simple [mdbook](https://crates.io/crates/mdbook) preprocessor designed to replace code blocks that look like:

    ```plantuml
    Bob -> Alice : Hello!
    ```

with an inline SVG rendered using [PlantUML](https://plantuml.com/):

```plantuml
Bob -> Alice : Hello!
```
I created this preprocessor because [mdbook-plantuml](https://crates.io/crates/mdbook-plantuml) wasn't working for me—specifically, mdbook-plantuml is currently incompatible with `mdbook watch` and `mbbook serve` because it triggers a [rebuild loop](https://github.com/sytsereitsma/mdbook-plantuml/issues/17).

This crate is quite simple and non-customizable at this point as it does all that I need it to for my own purposes. Feel free to fork and/or PR away though, and I'll be happy to include changes.

