use regex::{Regex, Captures};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Write, Result};
use crate::config;

// Função para salvar a linha no arquivo de saída
pub fn salvar_linha(caminho: &str, conteudo: &str) -> Result<()> {
    let mut arquivo = OpenOptions::new()
        .create(true)
        .append(true)
        .open(caminho)?;
    writeln!(arquivo, "{}", conteudo)?;
    Ok(())
}

// Função para substituir as tags na linha
pub fn atualizar_linha_dinamica(linha_original: &str, tabela_redirecionadora: Option<&HashMap<String, f64>>) -> String {
    let re = Regex::new(r"(V\d+:\[>R\$)(.*?)(<\]:-\[\d+\]\((.*?)\))").unwrap();
    let guia_nomes = config::obter_guia_traducoes();

    let resultado = re.replace_all(linha_original, |caps: &Captures| {
        let inicio = &caps[1];
        let label_impt = caps[4].trim();
        let final_tag = &caps[3];

        // 🛡️ PASSO 1: Valores especiais (ex: código município)
        if let Some(valor_fixo) = config::traduzir_valor_especial(label_impt) {
            return format!("{}{:.2}{}", inicio, format!("{:.2}", valor_fixo).replace('.', ","), final_tag);
        }

        // 🛡️ PASSO 2: Busca no mapa
        let label_busca = guia_nomes.get(label_impt).map(|s| s.as_str()).unwrap_or(label_impt);

        let novo_valor = match &tabela_redirecionadora {
            Some(tabela) => *tabela.get(label_busca).unwrap_or(&0.0),
            None => 0.0,
        };

        let valor_fmt = format!("{:.2}", novo_valor).replace('.', ",");
        format!("{}{}{}", inicio, valor_fmt, final_tag)
    });

    resultado.to_string()
}