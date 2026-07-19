// Parser separado da Main

#[derive(Debug)]
pub struct VariavelValor {
    pub nome: String,
    pub id_origem: u32,
    pub descricao: String,
}

#[derive(Debug)]
pub struct LayoutRegistro {
    pub bloco: String,
    pub natureza_despesa: String,
    pub id_sequencial: u32,
    pub variaveis: Vec<VariavelValor>,
    pub linha_destino: u32,
    pub coluna_destino: u32,
}
impl LayoutRegistro {
    /*pub fn parse_linha(linha: &str) -> Result<Self, String> {
        let campos: Vec<&str> = linha.trim().split(";").collect();

        if campos.len() < 6 {
            return Err(format!("Invalid linha: {}", linha));
        }

        let bloco = campos[0].to_string();
        let natureza_despesa = campos[1].to_string();

        let id_sequencial = campos[2]
            .parse::<u32>()
            .map_err(|e| format!("Invalid linha: {}", e))?;

        let variaveis = parse_bloco_variaveis(campos[3])?;
        let linha_destino = parse_coordenada(campos[4], 'L')
            .map_err(|e| format!("Invalid linha: {}", e))?;
        let coluna_destino = parse_coordenada(campos[5], 'C')
            .map_err(|e| format!("Invalid linha: {}", e))?;

        Ok(LayoutRegistro {
            bloco,
            natureza_despesa,
            id_sequencial,
            variaveis,
            linha_destino,
            coluna_destino,
        })
    }
}*/
pub fn parse_linha(linha: &str) -> Result<Self, String> {
    let campos: Vec<&str> = linha
        .trim()
        .split(';')
        .filter(|s| !s.trim().is_empty())
        .collect();

    if campos.len() < 5 {
        return Err(format!("Linha Invalida ou Muito curta: '{}'", linha));
    }
    let bloco = campos[0].to_string();
    let natureza_despesa = campos[1].to_string();
    /*let id_sequencial = campos[2]
        .parse::<u32>()
        .map_err(|e| format!("Erro ao ler ID Sequencial: {}", e))?;*/
    let id_sequencial = match campos[2].trim().parse::<u32>() {
        Ok(id) => id,
        Err(_) => return Err(format!("Linha Ignorada: ID '{}' nao é um numero válido", campos[2])),
    };
    let n = campos.len();
    let linha_destino = parse_coordenada(campos[n - 2], 'L')
        .map_err(|e| format!("Erro na Linha de destino: {}", e))?;
    let coluna_destino = parse_coordenada(campos[n - 1], 'C')
        .map_err(|e| format!("Erro na Linha de destino: {}", e))?;
    let pedacos_variaveis = &campos[3..n - 2];
    let variaveis = parse_multiplas_variaveis(pedacos_variaveis)?;

    Ok(LayoutRegistro {
        bloco,
        natureza_despesa,
        id_sequencial,
        variaveis,
        linha_destino,
        coluna_destino,
    })
    }
}
/*fn parse_coordenada (campo: &str, prefixo: char) -> Result<u32, String> {
    let padrao = format!("#{}", prefixo);
    if !campo.starts_with(&padrao) {
        return Err(format!("Invalid coordenada: {}", padrao));
    }
    campo[2..]
        .parse::<u32>()
        .map_err(|e| e.to_string())
}*/

fn parse_coordenada (campo: &str, prefixo: char) -> Result<u32, String> {
    let campo_limpo = campo.trim();
    let padrao = format!("#{}", prefixo);
    if !campo_limpo.starts_with(&padrao) {
        return Err(format!("Formato de coordenada esperado '{}X', recebido: '{}'", padrao, campo_limpo));
    }
    let numero_str = campo_limpo[2..].trim();
    if numero_str.is_empty() {
        return Err(format!("A Coordenada '{}' está vazia (sem numero).", padrao));
    }
    numero_str
        .parse::<u32>()
        .map_err(|e| format!("o Valor '{}' nao é uma coordenada numerica válida: {}", numero_str, e))
}

/*fn parse_bloco_variaveis(bloco: &str) -> Result<Vec<VariavelValor>, String> {
    let mut lista_variaveis = Vec::new();

    let partes: Vec<&str> = bloco.split("):").collect();
    for parte in partes {
        if parte.trim().is_empty() { continue; }
        let pedacos: Vec<&str> = parte.split(":[>R$<]:-").collect();
        if pedacos.len() != 2 {
            return Err(format!("Invalid variavelist parte '{}'", parte));
        }
        let nome = pedacos[0].to_string();
        let resto = pedacos[1];

        let inicio_id = resto.find('[').ok_or("Falta o caractere '['")? + 1;
        let fim_id = resto.find(']').ok_or("Falta o caractere ']'")?;

        let id_str = resto[inicio_id..fim_id].trim();();

        let id_origem = id_str
            .parse::<u32>()
            .map_err(|e| format!("ID Interno nao é um numero: {}", e))?;

        let inicio_desc = resto.find('(').ok_or("Falta o caractere '('")? + 1;
        let descricao = resto[inicio_desc..].trim_end_matches(')').to_string();

        lista_variaveis.push(VariavelValor {
            nome,
            id_origem,
            descricao,
        });
    }
    Ok(lista_variaveis)
}*/

fn parse_multiplas_variaveis(pedacos: &[&str]) -> Result<Vec<VariavelValor>, String> {
    let mut lista_variaveis = Vec::new();

    for pedaco in pedacos {
        let pedaco = pedaco.trim();
        if pedaco.is_empty() { continue; }

        let partes: Vec<&str> = pedaco.split(":[>R$<]:-").collect();
        if partes.len() != 2 {
            return Err(format!("Estrutura de variavel nao reconhecida: {}", pedaco));
        }
        let nome = partes[0].to_string();
        let resto = partes[1];
        let inicio_id = resto.find('[').ok_or("Falta '[' no ID da variavel")? + 1;
        let fim_id = resto.find(']').ok_or("Falta ']' no ID da variavel")?;
        let id_str = resto[inicio_id..fim_id].trim();
        let id_origem = id_str.parse::<u32>()
            .map_err(|e| format!("ID Interno '{}' nao e um numero: {}", id_str, e))?;
        let inicio_desc = resto.find('(').ok_or("Falta '(' na descricao da variavel")? + 1;
        let descricao = resto[inicio_desc..].trim_end_matches(')').trim().to_string();

        lista_variaveis.push(VariavelValor {
            nome,
            id_origem,
            descricao,
        });
    }
    Ok(lista_variaveis)
}