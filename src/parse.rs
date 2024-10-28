use pest::Parser;
use pest_derive::Parser;
use pest::error::Error;

#[derive(Parser)]
#[grammar = "syntax/chroma.pest"]
struct ChromaParser;

fn parse_chroma_file(file: &str) -> Result<(), Error<Rule>> {
    let chroma_file = ChromaParser::parse(Rule::file, file)?.next().unwrap();

    return Ok(())
}