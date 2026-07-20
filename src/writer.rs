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
        let valor_original = &caps[2]; // O valor que já estava na linha (ex: "224512,74")
        let final_tag = &caps[3];
        let label_impt = caps[4].trim();

        // 🛡️ PASSO 1: Valores especiais (ex: código município) - Mantém a regra original
        if let Some(valor_fixo) = config::traduzir_valor_especial(label_impt) {
            return format!("{}{:.2}{}", inicio, format!("{:.2}", valor_fixo).replace('.', ","), final_tag);
        }

        // 🛡️ PASSO 2: Busca no mapa
        let label_busca = guia_nomes.get(label_impt).map(|s| s.as_str()).unwrap_or(label_impt);

        // Lógica corrigida:
        // Se a tabela existe e tem o valor, usamos o novo.
        // CASO CONTRÁRIO, retornamos o valor_original que capturamos da linha.
        match &tabela_redirecionadora {
            Some(tabela) => {
                if let Some(&novo_valor) = tabela.get(label_busca) {
                    // Atualiza com o novo valor
                    let valor_fmt = format!("{:.2}", novo_valor).replace('.', ",");
                    format!("{}{}{}", inicio, valor_fmt, final_tag)
                } else {
                    // Mantém o valor original, sem alterar nada
                    format!("{}{}{}", inicio, valor_original, final_tag)
                }
            },
            None => {
                // Não há tabela de atualização, mantém tudo como está
                format!("{}{}{}", inicio, valor_original, final_tag)
            }
        }
    });

    resultado.to_string()
}