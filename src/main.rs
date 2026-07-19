mod parser;
mod reader;
mod xml_loader;
mod writer;

use parser::LayoutRegistro;
use std::fs;
use std::collections::HashMap;
use encoding_rs::WINDOWS_1252;

// Função de busca (deixe aqui fora do main)

fn limpar_codigo(cod: &str) -> String {
    // Remove pontos, traços e espaços extras
    cod.replace('.', "").replace('-', "").trim().to_string()
}
fn buscar_valor_por_aproximacao(
    cod_original: &str,
    mapa_valores: &HashMap<String, f64>
) -> Option<f64> {
    // Limpa o código de entrada
    let cod_limpo = limpar_codigo(cod_original);

    // 1. Tenta buscar o código exato (limpo)
    if let Some(&valor) = mapa_valores.get(&cod_limpo) {
        return Some(valor);
    }

    // 2. Tenta por prefixos (usando o código limpo)
    let comprimentos = [10, 8, 6, 4]; // Adicionei 4 para cobrir contas mais curtas
    for &len in &comprimentos {
        if cod_limpo.len() >= len {
            let prefixo = &cod_limpo[..len];

            if let Some((_, valor)) = mapa_valores.iter().find(|(cod, _)| cod.starts_with(prefixo)) {
                return Some(*valor);
            }
        }
    }
    None
}

fn main() {
    // ATO 1: Preparar os ingredientes (Carregar XML)
    // Dica: Certifique-se que o caminho aponta para o ARQUIVO, não para a pasta
    let mapa_valores = xml_loader::carregar_mapa_valores("./xml/DEMONSTRATIVO DA DESPESA DO ORGÃO.xml")
        .expect("Erro ao ler o XML");

    // ATO 2: Preparar a pasta
    let diretorio = "./entrada";
    let caminhos = fs::read_dir(diretorio).expect("Diretório de entrada não encontrado");

    // ATO 3: Cozinhar (Loop pelos arquivos)
    for entrada in caminhos {
        let caminho_arquivo = entrada.expect("Erro ao ler arquivo").path();

        // Só processa arquivos com a extensão .IMPT
        if caminho_arquivo.extension().and_then(|s| s.to_str()) == Some("IMPT") {
            println!("\nProcessando: {}", caminho_arquivo.display());

            if let Ok(linhas) = reader::ler_linhas(&caminho_arquivo) {
                for linha in linhas {
                    // Tenta transformar a linha em um REGISTRO
                    match LayoutRegistro::parse_linha(&linha) {
                        Ok(registro) => {
                            let valor_opcional = buscar_valor_por_aproximacao(&registro.natureza_despesa, &mapa_valores);

                            // 2. Decide o que fazer com a linha
                            let linha_final = if let Some(valor) = valor_opcional {
                                writer::atualizar_linha_impt(&linha, valor)
                            } else {
                                linha.clone()
                            };

                            // 3. Salva a linha (seja ela modificada ou original)
                            if let Err(e) = writer::salvar_linha("processado.impt", &linha_final) {
                                eprintln!("Erro ao gravar no arquivo: {}", e);
                            }
                        }
                        Err(_) => {
                            // Se a linha não é um registro válido, grava ela original
                            // Isso garante que o arquivo de saída tenha o mesmo tamanho que o de entrada
                            if let Err(e) = writer::salvar_linha("processado.impt", &linha) {
                                eprintln!("Erro ao gravar linha original: {}", e);
                            }
                        }
                    }
                }
            }
        }
    }
    println!("\nVarredura concluída.");
}