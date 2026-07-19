use serde::Deserialize;
use quick_xml::de::from_str;
use std::collections::HashMap;
use std::fs;
use encoding_rs::WINDOWS_1252;

#[derive(Debug, Deserialize)]
pub struct LinhaDetalhe {
    #[serde(rename = "CODNATUREZADESPESA")]
    pub cod: String, //mantem como string por ser valor mais longo
    #[serde(rename = "VALOREMPENHADO")]
    pub valor: String, //Leitura como String para trocar virgula por ponto
}
#[derive(Debug, Deserialize)]
struct Relatorio {
    #[serde(rename = "LinhaDetalhe", default)]
    pub linhas: Vec<LinhaDetalhe>,
}
pub fn carregar_mapa_valores(caminho: &str) -> Result<HashMap<String, f64>, String> {
    // 1. Lê como bytes brutos (não tenta interpretar como texto ainda)
    let bytes = fs::read(caminho).map_err(|e| e.to_string())?;

    // 2. Converte de Windows-1252 (ANSI) para String UTF-8
    let (res, _encoding_used, _had_errors) = WINDOWS_1252.decode(&bytes);
    let xml_content = res.into_owned();

    // 3. Agora processa o XML normalmente
    let relatorio: Relatorio = from_str(&xml_content)
        .map_err(|e| e.to_string())?;

    let mapa = relatorio.linhas
        .into_iter()
        .map(|l| {
            // Tratamento de vírgula para ponto
            let valor_f = l.valor.replace(',', ".").parse::<f64>().unwrap_or(0.0);
            (l.cod, valor_f)
        })
        .collect();

    Ok(mapa)
}