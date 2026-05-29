# SecureDrop Core

Aplicação Rust com Axum para receber arquivos, criptografá-los com age/X25519 e salvar no diretório `vault`.

## Como executar

```powershell
cargo run
```

## Estrutura

- `src/main.rs` — servidor e rotas
- `src/crypto.rs` — criptografia e gravação no cofre
- `src/frontend.rs` — página HTML de upload e tela de sucesso

## Requisitos

- Rust
- Cargo

## GitHub

Para publicar no GitHub:

```powershell
git init
git add .
git commit -m "Primeiro commit"
git branch -M main
git remote add origin https://github.com/SEU_USUARIO/SEU_REPOSITORIO.git
git push -u origin main
```
