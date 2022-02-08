/**
 * File              : main.rs
 * Author            : Kenton Hamaluik <kenton@hamaluik.ca>
 * Date              : 16.05.2020
 * Last Modified Date: 16.05.2020
 * Last Modified By  : Kenton Hamaluik <kenton@hamaluik.ca>
 */
/**
 * src/main.rs
 * Copyright (c) 2020 Kenton Hamaluik <kenton@hamaluik.ca>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use clap::{App, Arg, ArgMatches};
use mdbook::book::{Book, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use std::io;
use std::process;

fn make_app() -> App<'static> {
    App::new("puml")
        .about("A mdbook preprocessor which converts PlantUML code blocks into inline SVG")
        .subcommand(
            App::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor")
        )
}

fn main() {
    let matches = make_app().get_matches();

    // Users will want to construct their own preprocessor here
    let preprocessor = Puml::new();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

/// Pre-processor starter, taken straight out of the mdbook book
fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        // We should probably use the `semver` crate to check compatibility
        // here...
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

/// Check to see if we support the processor (puml only supports html right now),
/// taken straight out of the mdbook book
fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args.value_of("renderer").expect("Required argument");
    let supported = pre.supports_renderer(&renderer);

    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

pub struct Puml;

impl Puml {
    pub fn new() -> Puml {
        Puml
    }
}

impl Preprocessor for Puml {
    fn name(&self) -> &str { "puml" }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|book| {
            if let mdbook::BookItem::Chapter(chapter) = book {
                // TODO: better error handling...
                if let Err(e) = puml(chapter) {
                    eprintln!("PUML error: {:?}", e);
                }
            }
        });

        Ok(book)
    }
    
    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

/// Modify a chapter replacing all code blocks that look like:
///
///     ```plantuml
///     @startuml
///     Bob->Alice : hello
///     @enduml
///     ```
///
/// with inline SVG html
///
/// **Note**: there's a lot of `.expect()` calls in here, TODO: better error handling
fn puml(chapter: &mut Chapter) -> Result<(), Error> {
    use pulldown_cmark::{Parser, Event, Tag, CodeBlockKind, CowStr};

    // mini state machine for the current plantuml tag
    let mut puml: Option<String> = None;
    let events = Parser::new(&chapter.content)
        .filter_map(|event| {
            match &event {
                // a code block for the `plantuml` language was started
                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(CowStr::Borrowed("plantuml")))) => {
                    puml = Some("".to_owned());
                    None
                },
                // a code block for the `plantuml` language was ended
                Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(CowStr::Borrowed("plantuml")))) => {
                    // extract the contents of the diagram
                    let puml_src = puml.take().expect("puml was started");

                    // launch plantuml to render to SVG
                    use std::process::{Command, Stdio};
                    use std::io::Write;
                    let mut child = match Command::new("plantuml")
                        .arg("-tsvg")
                        .arg("-nometadata")
                        .arg("-pipe")
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn() {
                            Ok(c) => c,
                            Err(e) => {
                                eprintln!("failed to launch plantuml, not rendering plantuml block: {:?}", e);
                                return None;
                            }
                        };

                    let stdin = child.stdin.as_mut().expect("valid plantuml stdin");
                    stdin.write_all(puml_src.as_ref()).expect("can write to plantuml stdin");
                    let output = child.wait_with_output().expect("can launch plantuml");
                    
                    // check for failure
                    if !output.status.success() {
                        eprintln!("plantuml failed, exit code: {:?}", output.status.code());

                        eprintln!("plantuml STDOUT:");
                        eprintln!("{}", String::from_utf8_lossy(output.stdout.as_ref()));
                        eprintln!("plantuml STDERR:");
                        eprintln!("{}", String::from_utf8_lossy(output.stdout.as_ref()));
                        eprintln!("/plantuml output");

                        return None;
                    }

                    // extact the svg, getting rid of the starting xml tag
                    let svg: String = String::from_utf8(output.stdout).expect("valid utf-8");
                    let svg = svg.replace(r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#, "");

                    // and wrap it up in a figure and emit it
                    Some(Event::Html(CowStr::from(format!("<figure>{}</figure>\n\n", svg))))
                },
                // intercept text events if we're currently in the code block state
                Event::Text(txt) => {
                    if let Some(puml) = puml.as_mut() {
                        puml.push_str(&txt);
                        None
                    }
                    else {
                        Some(event)
                    }
                },
                // don't touch other events
                _ => Some(event),
            }
        });

    let mut buf = String::with_capacity(chapter.content.len());
    pulldown_cmark_to_cmark::cmark(events, &mut buf).expect("can re-render cmark");
    chapter.content = buf;

    Ok(())
}

