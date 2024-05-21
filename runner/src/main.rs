use std::{
    env::args,
    fs::File,
    io::{BufReader, Result},
};

use mpls_tool::parser::Parser;
fn main() -> Result<()> {
    let mut args = args();
    let path = args.nth(1).expect("Path not provided as first argument");

    let f = File::open(path)?;
    let reader = &mut BufReader::new(f);
    let mut parser = Parser::default();
    parser.read_header(reader)?;
    parser.read_playlist_header(reader)?;
    parser.read_play_item_entries(reader)?;
    parser.read_chapters_header(reader)?;
    parser.read_chapters(reader)?;

    Ok(())
}
