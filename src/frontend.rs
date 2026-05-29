use axum::{
    response::{Html, IntoResponse},
};

pub async fn exibir_formulario() -> Html<&'static str> {
    Html(r#"
        <!DOCTYPE html>
        <html lang="pt-BR">
        <head>
            <meta charset="UTF-8">
            <title>SecureDrop - Portal</title>
            <style>
                body { background-color: #050505; color: #00ff00; font-family: monospace; display: flex; justify-content: center; align-items: center; height: 100vh; margin: 0; }
                .container { border: 1px solid #333; padding: 2rem; background: #0a0a0a; text-align: center; border-radius: 8px; }
                input[type="file"] { margin: 1rem 0; padding: 1rem; border: 1px dashed #444; width: 100%; box-sizing: border-box; color: #00ff00; }
                button { background: #00ff00; color: #000; border: none; padding: 0.8rem 2rem; cursor: pointer; font-weight: bold; width: 100%; border-radius: 4px; transition: 0.2s; }
                button:hover { background: #00cc00; }
            </style>
        </head>
        <body>
            <div class="container">
                <h2>[ SATAN ]</h2>
                <p>Conexão segura. Seus arquivos serão criptografados antes do armazenamento.</p>
                <form action="/submit" method="POST" enctype="multipart/form-data">
                    <input type="file" name="file" required>
                    <button type="submit">CRIPTOGRAFAR E ENVIAR</button>
                </form>
            </div>
        </body>
        </html>
    "#)
}

pub fn gerar_tela_de_sucesso() -> axum::response::Response {
    Html(r#"
        <body style="background:#050505;color:#00ff00;font-family:monospace;display:flex;justify-content:center;align-items:center;height:100vh;flex-direction:column;">
            <h2>[ OK ] Operação concluída.</h2>
            <p>O arquivo foi guardado no cofre com segurança.</p>
            <a href="/" style="color:#00ff00; margin-top:20px; text-decoration:none; border: 1px solid #00ff00; padding: 10px 20px;">Fazer novo envio</a>
        </body>
    "#)
        .into_response()
}