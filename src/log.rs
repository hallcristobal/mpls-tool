use std::io::Write;

pub use termcolor::Color;
use termcolor::{BufferWriter, ColorChoice, ColorSpec, WriteColor};

pub fn key_val_print(color: Option<Color>, key: &str, val: &str) {
    let bufwtr = BufferWriter::stderr(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();

    buffer
        .set_color(
            ColorSpec::new()
                .set_fg(Some(color.unwrap_or(Color::Green)))
                .set_bold(true),
        )
        .ok();
    write!(&mut buffer, "{:>30}", key).ok();

    buffer.reset().ok();
    writeln!(&mut buffer, " {}", val).ok();
    bufwtr.print(&buffer).ok();
}
