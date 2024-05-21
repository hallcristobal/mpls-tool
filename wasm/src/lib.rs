use std::io::BufReader;

use mpls_tool::parser::Parser;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn analyze_mpls(data: Vec<u8>) -> bool {
    let reader = BufReader::new(data.as_slice());
    run(reader).is_ok()
}

fn run(mut reader: BufReader<&[u8]>) -> std::io::Result<()> {
    let mut parser = Parser::default();
    let reader = &mut reader;
    parser.read_header(reader)?;
    parser.read_playlist_header(reader)?;
    parser.read_play_item_entries(reader)?;
    parser.read_chapters_header(reader)?;
    parser.read_chapters(reader)?;

    Ok(())
}
