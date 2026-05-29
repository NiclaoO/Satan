use age::x25519::Recipient;
use std::str::FromStr;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::io::Write;
use uuid::Uuid;

const UPLOAD_DIR: &str = "./vault";

pub async fn trancar_no_cofre(dados: Vec<u8>) -> Result<(), ()> { 
    println!("[COFRE] Começando a criptografia de {} bytes...", dados.len());

    // O código agora caça a chave no ambiente em vez de ler do próprio texto
    let chave_publica = std::env::var("VAULT_PUB_KEY").map_err(|_| {
        eprintln!("[ERRO FATAL] A variável VAULT_PUB_KEY não foi encontrada no ambiente!");
    })?;

    let chave = Recipient::from_str(&chave_publica).map_err(|erro| {
        eprintln!("[ERRO] A chave pública não pôde ser lida: {}", erro);
    })?;

    let nome_arquivo = Uuid::new_v4().to_string();
    let caminho = format!("{}/{}.enc", UPLOAD_DIR, nome_arquivo);

    let encriptador = age::Encryptor::with_recipients(vec![Box::new(chave)]).ok_or_else(|| {
        eprintln!("[ERRO] Não foi possível iniciar a criptografia Age.");
    })?;

    let mut dados_criptografados = Vec::new();

    {
        let mut fluxo = encriptador.wrap_output(&mut dados_criptografados).map_err(|erro| {
            eprintln!("[ERRO] Não foi possível abrir o fluxo de criptografia: {:?}", erro);
        })?;

        fluxo.write_all(&dados).map_err(|erro| {
            eprintln!("[ERRO] Não foi possível criptografar os dados: {}", erro);
        })?;

        fluxo.finish().map_err(|erro| {
            eprintln!("[ERRO] Não foi possível finalizar a criptografia: {:?}", erro);
        })?;
    }

    println!("[COFRE] Salvando o arquivo criptografado em: {}", caminho);

    let mut arquivo = File::create(&caminho).await.map_err(|erro| {
        eprintln!("[ERRO] Não foi possível criar o arquivo no cofre: {}", erro);
    })?;

    arquivo.write_all(&dados_criptografados).await.map_err(|erro| {
        eprintln!("[ERRO] Não foi possível gravar o arquivo criptografado: {}", erro);
    })?;

    println!("[COFRE] Arquivo salvo com sucesso.");
    Ok(())
}