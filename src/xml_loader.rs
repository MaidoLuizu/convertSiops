use serde::Deserialize;
use quick_xml::de::from_str;
use std::collections::HashMap;
use std::fs;
use encoding_rs::WINDOWS_1252;

#[derive(Debug, Deserialize)]
pub struct LinhaDetalhe {
    #[serde(rename = "CODNATUREZADESPESA")]
    pub cod: String,
    #[serde(rename = "VALORAUTORIZADO", default)] pub dotacao_inicial: String,
    #[serde(rename = "VALORATUALIZADO", default)] pub dotacao_atualizada: String,
    #[serde(rename = "VALOREMPENHADO", default)] pub valor_empenhado: String,
    #[serde(rename = "VALORLIQUIDADO", default)] pub valor_liquidado: String,
    #[serde(rename = "VALORPAGO", default)] pub valor_pago: String,
    #[serde(rename = "VALORRESTOSAPAGAR", default)] pub inscritas_rp_np: String,
}

#[derive(Debug, Deserialize)]
struct Relatorio {
    #[serde(rename = "LinhaDetalhe", default)]
    pub linhas: Vec<LinhaDetalhe>,
}

fn para_f64(valor_str: &str) -> f64 {
    valor_str.replace(',', ".").parse::<f64>().unwrap_or(0.0)
}

/// Remove pontos, traços e limpa TODOS os zeros do final para isolar o nível real da conta
fn normalizar_codigo(cod: &str) -> String {
    let limpo = cod.replace('.', "").replace('-', "").trim().to_string();
    let mut s = limpo;
    while s.ends_with('0') && s.len() > 2 {
        s.pop();
    }
    s
}

pub fn carregar_mapa_valores(caminho: &str) -> Result<HashMap<String, HashMap<String, f64>>, String> {
    let bytes = fs::read(caminho).map_err(|e| e.to_string())?;
    let (res, _encoding_used, _had_errors) = WINDOWS_1252.decode(&bytes);
    let xml_content = res.into_owned();

    let relatorio: Relatorio = from_str(&xml_content).map_err(|e| e.to_string())?;

    // Mapa final que acumularemos os valores
    let mut mapa_consolidado: HashMap<String, HashMap<String, f64>> = HashMap::new();

    for l in relatorio.linhas {
        let cod_chave = normalizar_codigo(&l.cod);
        println!("DEBUG: Salvando no mapa a chave: '|{}|' (Bytes: {:?})", cod_chave, cod_chave.as_bytes());
        // Pega ou cria a tabela interna para essa conta
        let tabela = mapa_consolidado.entry(cod_chave).or_insert_with(HashMap::new);

        // Soma cada valor individualmente
        *tabela.entry("Dotação Inicial".to_string()).or_insert(0.0) += para_f64(&l.dotacao_inicial);
        *tabela.entry("Dotação Atualizada".to_string()).or_insert(0.0) += para_f64(&l.dotacao_atualizada);
        *tabela.entry("Despesas Empenhadas".to_string()).or_insert(0.0) += para_f64(&l.valor_empenhado);
        *tabela.entry("Despesas Liquidadas".to_string()).or_insert(0.0) += para_f64(&l.valor_liquidado);
        *tabela.entry("Despesas Pagas".to_string()).or_insert(0.0) += para_f64(&l.valor_pago);
        *tabela.entry("Inscritas em Restos a Pagar Não Processados".to_string()).or_insert(0.0) += para_f64(&l.inscritas_rp_np);
    }

    Ok(mapa_consolidado)
}