use axum::response::{Html, IntoResponse};

pub async fn exibir_formulario() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <title>SecureDrop - Portal</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@300;400;500;600&family=Share+Tech+Mono&display=swap" rel="stylesheet">
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@tabler/icons-webfont@latest/tabler-icons.min.css">
    <style>
        *, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }

        body {
            background: #080c0a;
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            font-family: 'JetBrains Mono', monospace;
            padding: 2rem;
            position: relative;
            overflow: hidden;
        }

        body::before {
            content: '';
            position: fixed; inset: 0;
            background-image:
                linear-gradient(rgba(0,200,100,0.03) 1px, transparent 1px),
                linear-gradient(90deg, rgba(0,200,100,0.03) 1px, transparent 1px);
            background-size: 32px 32px;
            pointer-events: none;
        }

        body::after {
            content: '';
            position: fixed; inset: 0;
            background: repeating-linear-gradient(
                0deg,
                transparent, transparent 2px,
                rgba(0,0,0,0.08) 2px, rgba(0,0,0,0.08) 4px
            );
            pointer-events: none;
            z-index: 1;
        }

        .card {
            position: relative;
            z-index: 2;
            width: 100%;
            max-width: 380px;
            border: 1px solid rgba(0,200,100,0.2);
            background: rgba(8,14,10,0.97);
            border-radius: 2px;
            box-shadow: 0 0 40px rgba(0,200,100,0.04), inset 0 0 60px rgba(0,0,0,0.3);
        }

        .corner {
            position: absolute;
            width: 10px; height: 10px;
            border-color: rgba(0,200,100,0.5);
            border-style: solid;
        }
        .corner.tl { top: -1px; left: -1px; border-width: 2px 0 0 2px; }
        .corner.tr { top: -1px; right: -1px; border-width: 2px 2px 0 0; }
        .corner.bl { bottom: -1px; left: -1px; border-width: 0 0 2px 2px; }
        .corner.br { bottom: -1px; right: -1px; border-width: 0 2px 2px 0; }

        .titlebar {
            display: flex;
            align-items: center;
            gap: 8px;
            padding: 10px 16px;
            border-bottom: 1px solid rgba(0,200,100,0.12);
            background: rgba(0,200,100,0.03);
        }

        .dot {
            width: 8px; height: 8px;
            border-radius: 50%;
            background: rgba(0,200,100,0.2);
            border: 1px solid rgba(0,200,100,0.35);
        }
        .dot.active {
            background: #00c864;
            border-color: #00c864;
            box-shadow: 0 0 6px #00c864;
        }

        .titlebar-text {
            font-size: 11px;
            color: rgba(0,200,100,0.4);
            letter-spacing: 0.15em;
            flex: 1;
            text-align: center;
            font-family: 'Share Tech Mono', monospace;
        }

        .body { padding: 2rem 1.75rem 1.75rem; }

        .logo { text-align: center; margin-bottom: 1.5rem; }

        .logo-bracket {
            display: block;
            font-size: 10px;
            letter-spacing: 0.3em;
            color: rgba(0,200,100,0.3);
            margin-bottom: 6px;
            font-family: 'Share Tech Mono', monospace;
        }

        .logo-name {
            display: block;
            font-size: 24px;
            font-weight: 600;
            color: #00c864;
            letter-spacing: 0.25em;
            font-family: 'Share Tech Mono', monospace;
            text-shadow: 0 0 20px rgba(0,200,100,0.3);
        }

        .status {
            display: flex;
            align-items: center;
            gap: 8px;
            font-size: 11px;
            color: rgba(0,200,100,0.5);
            letter-spacing: 0.05em;
            margin-bottom: 1.5rem;
            padding: 8px 12px;
            border: 1px solid rgba(0,200,100,0.1);
            background: rgba(0,200,100,0.02);
            border-radius: 2px;
        }

        .pulse {
            width: 6px; height: 6px;
            border-radius: 50%;
            background: #00c864;
            flex-shrink: 0;
            animation: pulse 2s ease-in-out infinite;
        }

        @keyframes pulse {
            0%, 100% { opacity: 1; }
            50% { opacity: 0.3; }
        }

        .drop-zone {
            border: 1px dashed rgba(0,200,100,0.2);
            border-radius: 2px;
            padding: 1.5rem 1rem;
            text-align: center;
            background: rgba(0,200,100,0.015);
            cursor: pointer;
            transition: all 0.2s;
            margin-bottom: 10px;
        }

        .drop-zone:hover {
            border-color: rgba(0,200,100,0.4);
            background: rgba(0,200,100,0.04);
        }

        .drop-icon {
            font-size: 22px;
            color: rgba(0,200,100,0.3);
            margin-bottom: 8px;
        }

        .drop-label {
            display: block;
            font-size: 12px;
            color: rgba(0,200,100,0.5);
            letter-spacing: 0.08em;
            margin-bottom: 4px;
        }

        .drop-hint {
            font-size: 10px;
            color: rgba(0,200,100,0.25);
            letter-spacing: 0.05em;
        }

        input[type="file"] { display: none; }

        .filename {
            font-size: 11px;
            color: rgba(0,200,100,0.6);
            text-align: center;
            margin-bottom: 10px;
            min-height: 16px;
            letter-spacing: 0.05em;
            display: none;
        }

        .filename.visible { display: block; }

        button {
            width: 100%;
            background: transparent;
            border: 1px solid rgba(0,200,100,0.4);
            color: #00c864;
            font-family: 'JetBrains Mono', monospace;
            font-size: 12px;
            font-weight: 500;
            letter-spacing: 0.2em;
            padding: 12px;
            cursor: pointer;
            transition: all 0.15s;
            border-radius: 2px;
            text-transform: uppercase;
            margin-bottom: 1rem;
        }

        button:hover {
            background: rgba(0,200,100,0.08);
            border-color: #00c864;
            box-shadow: 0 0 16px rgba(0,200,100,0.1);
        }

        button:active { opacity: 0.7; }

        .footer-items {
            display: flex;
            gap: 16px;
            font-size: 10px;
            color: rgba(0,200,100,0.2);
            letter-spacing: 0.08em;
            justify-content: center;
        }

        .footer-items span::before { content: '// '; }
    </style>
</head>
<body>
    <div class="card">
        <div class="corner tl"></div>
        <div class="corner tr"></div>
        <div class="corner bl"></div>
        <div class="corner br"></div>

        <div class="titlebar">
            <div class="dot active"></div>
            <div class="dot"></div>
            <div class="dot"></div>
            <span class="titlebar-text">secure-drop — terminal v2.1</span>
        </div>

        <div class="body">
            <div class="logo">
                <span class="logo-bracket">◈ SISTEMA ATIVO ◈</span>
                <span class="logo-name">SATAN</span>
            </div>

            <div class="status">
                <div class="pulse"></div>
                <span>conexão encriptada · AES-256 · canal seguro</span>
            </div>

            <form action="/submit" method="POST" enctype="multipart/form-data">
                <div class="drop-zone" onclick="document.getElementById('fileInput').click()">
                    <div class="drop-icon"><i class="ti ti-file-upload"></i></div>
                    <span class="drop-label">selecionar arquivo</span>
                    <span class="drop-hint">qualquer formato · sem rastros</span>
                </div>

                <div class="filename" id="fileName"></div>
                <input type="file" id="fileInput" name="file" required>

                <button type="submit">
                    <i class="ti ti-lock" style="margin-right:8px;vertical-align:-2px;"></i>
                    criptografar e enviar
                </button>
            </form>

            <div class="footer-items">
                <span>zero logs</span>
                <span>end-to-end</span>
                <span>anônimo</span>
            </div>
        </div>
    </div>

    <script>
        const input = document.getElementById('fileInput');
        const label = document.getElementById('fileName');
        input.addEventListener('change', () => {
            if (input.files.length > 0) {
                label.textContent = '> ' + input.files[0].name;
                label.classList.add('visible');
            }
        });
    </script>
</body>
</html>
    "#)
}

pub fn gerar_tela_de_sucesso() -> axum::response::Response {
    Html(r#"
<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <title>SecureDrop - OK</title>
    <link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500&family=Share+Tech+Mono&display=swap" rel="stylesheet">
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@tabler/icons-webfont@latest/tabler-icons.min.css">
    <style>
        * { box-sizing: border-box; margin: 0; padding: 0; }
        body {
            background: #080c0a;
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            font-family: 'JetBrains Mono', monospace;
        }
        body::before {
            content: '';
            position: fixed; inset: 0;
            background-image:
                linear-gradient(rgba(0,200,100,0.03) 1px, transparent 1px),
                linear-gradient(90deg, rgba(0,200,100,0.03) 1px, transparent 1px);
            background-size: 32px 32px;
            pointer-events: none;
        }
        .card {
            position: relative;
            z-index: 2;
            text-align: center;
            padding: 3rem 2.5rem;
            border: 1px solid rgba(0,200,100,0.2);
            background: rgba(8,14,10,0.97);
            border-radius: 2px;
            max-width: 360px;
            width: 100%;
        }
        .check-icon {
            font-size: 40px;
            color: #00c864;
            margin-bottom: 1.25rem;
            opacity: 0.8;
        }
        .tag {
            font-size: 10px;
            color: rgba(0,200,100,0.35);
            letter-spacing: 0.3em;
            font-family: 'Share Tech Mono', monospace;
            margin-bottom: 8px;
        }
        h2 {
            font-size: 18px;
            font-weight: 500;
            color: #00c864;
            letter-spacing: 0.1em;
            margin-bottom: 10px;
        }
        p {
            font-size: 12px;
            color: rgba(0,200,100,0.4);
            letter-spacing: 0.05em;
            line-height: 1.7;
            margin-bottom: 2rem;
        }
        a {
            display: inline-block;
            color: #00c864;
            font-size: 11px;
            letter-spacing: 0.2em;
            text-decoration: none;
            border: 1px solid rgba(0,200,100,0.3);
            padding: 10px 24px;
            border-radius: 2px;
            transition: all 0.15s;
            text-transform: uppercase;
        }
        a:hover {
            background: rgba(0,200,100,0.08);
            border-color: #00c864;
        }
    </style>
</head>
<body>
    <div class="card">
        <div class="check-icon"><i class="ti ti-shield-check"></i></div>
        <div class="tag">[ OPERAÇÃO CONCLUÍDA ]</div>
        <h2>arquivo seguro</h2>
        <p>O arquivo foi cifrado e armazenado<br>no cofre com sucesso.</p>
        <a href="/">novo envio</a>
    </div>
</body>
</html>
    "#)
        .into_response()
}