mod crypto;
mod frontend; 

use axum::{
    extract::Multipart,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::limit::RequestBodyLimitLayer;
use std::path::Path;

const MAX_UPLOAD_SIZE: usize = 100 * 1024 * 1024; 
const UPLOAD_DIR: &str = "./vault";

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    preparar_cofre().await;

    println!("Iniciando protocolos de segurança...");
    println!("Servidor rodando na porta 8080. Módulos carregados com sucesso.");

    let app = Router::new()
        .route("/", get(frontend::exibir_formulario))
        .route("/submit", post(receber_arquivo))
        .layer(RequestBodyLimitLayer::new(MAX_UPLOAD_SIZE));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    axum::serve(listener, app).await.unwrap();
}

async fn preparar_cofre() {
    if !Path::new(UPLOAD_DIR).exists() {
        tokio::fs::create_dir(UPLOAD_DIR)
            .await
            .expect("Falha crítica: Não foi possível criar a pasta do cofre.");
    }
}

async fn receber_arquivo(mut multipart: Multipart) -> impl IntoResponse {
    println!("Requisição recebida na rota /submit.");

    while let Some(campo) = match multipart.next_field().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!(" [ERRO REDE] Falha ao ler a requisição Multipart: {:?}", e);
            return StatusCode::BAD_REQUEST.into_response();
        }
    } {
        let nome_arquivo = campo.file_name().unwrap_or("sem_nome").to_string();
        println!("Extraindo o arquivo: '{}'", nome_arquivo);

        let dados_puros = match campo.bytes().await {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!(" [ERRO REDE] Falha ao baixar os bytes do arquivo: {:?}", e);
                return StatusCode::BAD_REQUEST.into_response();
            }
        };

        println!("Arquivo carregado na memória ({} bytes). Enviando para criptografia...", dados_puros.len());

        if crypto::trancar_no_cofre(dados_puros.to_vec()).await.is_err() {
            eprintln!("A criptografia falhou. Retornando erro 500.");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    println!("Processo concluído. Enviando mensagem de sucesso.");
    frontend::gerar_tela_de_sucesso()
}