use regex::Regex;
use std::fs::OpenOptions;
use std::io::{Write, Result};

pub fn atualizar_linha_impt(linha_original: &str, novo_valor: f64) -> String {
    // 1. Formata o número (ex: 224512,74)
    let valor_fmt = format!("{:.2}", novo_valor).replace('.', ",");

    // 2. Cria o fragmento novo
    let novo_fragmento = format!("V1:[>R${}<]", valor_fmt);

    // 3. Em vez de Regex, procuramos a posição da tag V1 e substituímos o bloco
    // Isso é mais seguro para sistemas legados
    if let Some(start) = linha_original.find("V1:[>R$") {
        if let Some(end) = linha_original[start..].find("<]") {
            let real_end = start + end + 2; // +2 para incluir o "<]"

            // Monta a linha: [tudo antes da tag] + [novo valor] + [tudo depois da tag]
            return format!("{}{}{}",
                           &linha_original[..start],
                           novo_fragmento,
                           &linha_original[real_end..]
            );
        }
    }

    // Se não achar a tag, retorna a original
    linha_original.to_string()
}
pub fn salvar_linha(caminho: &str, conteudo: &str) -> Result<()> {
    let mut arquivo = OpenOptions::new()
        .create(true)
        .append(true)
        .open(caminho)?;

    writeln!(arquivo, "{}", conteudo)?;
    Ok(())
}