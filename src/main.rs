use mago_formatter::settings::FormatSettings;
use mago_interner::ThreadedInterner;
use mago_source::Source;
use unicode_segmentation::UnicodeSegmentation;

const ATTRIBUTE_PREFIX: &str = "<?php ";
const ATTRIBUTE_SUFFIX: &str = "\nfunction dummy()\n{\n}";

fn main() -> std::io::Result<()> {
    let interner = ThreadedInterner::new();
    let files = std::env::args().skip(1).collect::<Vec<_>>();

    for file in files {
        if !std::fs::metadata(&file)?.is_file() {
            eprintln!("Skipping {file} as it is not a file ?");
            continue;
        }
        println!("Processing file {file}");

        let formatted_content = std::fs::read_to_string(&file)?
            .lines()
            .enumerate()
            .map(|(i, line)| {
                if !line.trim().starts_with("#[") || line.len() < 80 {
                    return format!("{line}\n");
                }

                let input = format!("{ATTRIBUTE_PREFIX}{}{ATTRIBUTE_SUFFIX}", line.trim());
                let source = Source::standalone(&interner, &format!("{file}:{i}"), &input);
                let (program, errs) = mago_parser::parse(
                    &interner,
                    mago_lexer::input::Input::new(source.identifier, input.as_bytes()),
                );

                if let Some(errs) = errs {
                    eprintln!("Could not format line {file}:{i} ? ({errs})");

                    return format!("{line}\n");
                }

                mago_formatter::format(&interner, &source, &program, FormatSettings::default())
                    .to_string()
                    .graphemes(true)
                    .skip(ATTRIBUTE_PREFIX.len())
                    .collect::<String>()
                    .graphemes(true)
                    .rev()
                    .skip(ATTRIBUTE_SUFFIX.len())
                    .collect::<String>()
                    .graphemes(true)
                    .rev()
                    .collect::<String>()
                    .lines()
                    .map(|line| format!("    {line}\n"))
                    .collect()
            })
            .collect::<String>();

        std::fs::write(file, formatted_content)?;
    }

    Ok(())
}
