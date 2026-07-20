mod parser;
mod reader;
mod xml_loader;
mod writer;
mod config;

use parser::LayoutRegistro;
use std::fs;
use std::collections::HashMap;

/// Função de normalização idêntica à do config.rs
fn normalizar(cod: &str) -> String {
    cod.replace('.', "").replace('-', "").trim().to_string()
}

fn main() {
    // 1. Limpeza do arquivo de saída anterior
    let _ = fs::remove_file("processado.impt");

    // 2. Carregamento dos dados
    println!("Carregando dados do XML...");
    let mapa_xml = xml_loader::carregar_mapa_valores("./xml/DEMONSTRATIVO DA DESPESA DO ORGÃO.xml")
        .expect("Erro ao ler o XML");

    let guia_codigos = config::obter_guia_codigos_map();

    // 3. Processamento dos arquivos
    let diretorio = "./entrada";
    if let Ok(caminhos) = fs::read_dir(diretorio) {
        for entrada in caminhos.flatten() {
            let caminho_arquivo = entrada.path();

            // Processa apenas arquivos .IMPT
            if caminho_arquivo.extension().and_then(|s| s.to_str()) == Some("IMPT") {
                println!("Processando arquivo: {}", caminho_arquivo.display());

                if let Ok(linhas) = reader::ler_linhas(&caminho_arquivo) {
                    for linha in linhas {
                        let linha_final = match LayoutRegistro::parse_linha(&linha) {
                            Ok(registro) => {
                                let impt_cod = normalizar(&registro.natureza_despesa);

                                // ACUMULAÇÃO: Busca todas as chaves XML mapeadas e soma seus valores
                                let mut totais_acumulados: HashMap<String, f64> = HashMap::new();
                                let mut encontrou_mapeamento = false;

                                if let Some(entries) = guia_codigos.get(&impt_cod) {
                                    encontrou_mapeamento = true;
                                    for entry in entries {
                                        if let Some(tabela_xml) = mapa_xml.get(&entry.xml_key) {
                                            for (conta, valor) in tabela_xml {
                                                // Lógica de filtro:
                                                // Se a lista de permitidos estiver vazia, aceita tudo.
                                                // Caso contrário, verifica se 'conta' está na lista.
                                                let permitido = entry.allowed_columns.is_empty() ||
                                                    entry.allowed_columns.contains(conta);

                                                if permitido {
                                                    *totais_acumulados.entry(conta.clone()).or_insert(0.0) += valor;
                                                }
                                            }
                                        } else {
                                            println!("  -> [AVISO] Chave mapeada '{}' não encontrada no XML.", entry.xml_key);
                                        }
                                    }
                                }

                                // DEBUG: Se não houver mapeamento para o código IMPT
                                if !encontrou_mapeamento {
                                    println!("  -> [DEBUG] Nenhum mapeamento para: {}", impt_cod);
                                }

                                // Atualiza a linha com os valores acumulados
                                writer::atualizar_linha_dinamica(&linha, Some(&totais_acumulados))
                            }
                            // Se a linha não for válida, mantém a original
                            Err(_) => linha.to_string(),
                        };

                        // 4. Salva a linha processada
                        if let Err(e) = writer::salvar_linha("processado.impt", &linha_final) {
                            eprintln!("Erro ao gravar: {}", e);
                        }
                    }
                }
            }
        }
    }
    println!("\nProcessamento concluído. Verifique o arquivo 'processado.impt'.");
}