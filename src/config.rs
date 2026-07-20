// src/config.rs
use std::collections::HashMap;
use std::fs::File;
use lazy_static::lazy_static;

// Função auxiliar interna para garantir que o CSV seja lido já normalizado
pub fn normalizar(cod: &str) -> String {
    // Apenas limpa a formatação visual (pontos e traços),
    // preservando todos os dígitos, inclusive os zeros.
    cod.replace('.', "").replace('-', "").trim().to_string()
}

pub struct MappingEntry {
    pub xml_key: String,
    pub allowed_columns: Vec<String>,
}

lazy_static! {
    pub static ref GUIA_CODIGOS: HashMap<String, Vec<MappingEntry>> = {
        let mut de_para: HashMap<String, Vec<MappingEntry>> = HashMap::new();
        if let Ok(file) = File::open("mapeamento_contas.csv") {
            let mut rdr = csv::ReaderBuilder::new().has_headers(true).from_reader(file);
            for result in rdr.records() {
                if let Ok(reg) = result {
                    let impt = normalizar(&reg[0]);
                    let xml = normalizar(&reg[1]);

                    // Lê a 3ª coluna se existir
                    let colunas_str = reg.get(2).unwrap_or("").to_string();
                    let allowed: Vec<String> = if colunas_str.is_empty() {
                        vec![] // Se vazio, permite tudo
                    } else {
                        colunas_str.split('|').map(|s| s.trim().to_string()).collect()
                    };

                    de_para.entry(impt).or_default().push(MappingEntry {
                        xml_key: xml,
                        allowed_columns: allowed,
                    });
                }
            }
        }
        de_para
    };
}

// Mude o retorno da função para Vec<String>
pub fn obter_guia_codigos_map() -> &'static HashMap<String, Vec<MappingEntry>> {
    &GUIA_CODIGOS
}

// A palavra 'pub' aqui é obrigatória!
pub fn obter_guia_traducoes() -> HashMap<String, String> {
    let mut de_para = HashMap::new();

    // Insira suas traduções aqui
    de_para.insert("Dotação Inicial".to_string(), "Dotação Inicial".to_string());
    de_para.insert("Dotação Atualizada".to_string(), "Dotação Atualizada".to_string());
    // ... adicione as outras conforme necessário

    de_para
}

// Também certifique-se de que esta também seja 'pub'
pub fn traduzir_valor_especial(label: &str) -> Option<f64> {
    match label {
        "Código do Município" => Some(4105204.0),
        _ => None,
    }
}