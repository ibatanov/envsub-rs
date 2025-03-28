use std::{fs, io::{self, Read}};

use envsub_rs::{cli::parse_args, errors::EnvSubstError, substitutor::substitute};

fn main() -> Result<(), EnvSubstError> {
    // Парсим аргументы командной строки
    let config = parse_args().map_err(|err| {
        eprintln!("Error parsing arguments: {}", err);
        std::process::exit(1);
    })?;

    // Читаем содержимое входного файла или stdin
    let input = match config.input {
        Some(input_file) => fs::read_to_string(input_file).map_err(|err| {
            EnvSubstError::ParsingError(format!("Failed to read input file: {}", err))
        })?,
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).map_err(|err| {
                EnvSubstError::ParsingError(format!("Failed to read from stdin: {}", err))
            })?;
            buffer
        }
    };

    // Выполняем подстановку переменных
    let output = substitute(&input, config.no_unset, config.no_empty)?;

    // Записываем результат в выходной файл или stdout
    match config.output {
        Some(output_file) => fs::write(output_file, output).map_err(|err| {
            EnvSubstError::ParsingError(format!("Failed to write to output file: {}", err))
        })?,
        None => println!("{}", output),
    }

    Ok(())
}
