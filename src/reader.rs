use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn ler_linhas<P>(caminho: P) -> io::Result<Vec<String>>
where P: AsRef<Path>, {
    let arquivo = File::open(caminho)?;
    let leitor = io::BufReader::new(arquivo);

    //filtro para linhas vazias
    let linhas = leitor.lines()
        .filter_map(Result::ok)
        .filter(|linha| !linha.trim().is_empty())
        .collect();

    Ok(linhas)
}