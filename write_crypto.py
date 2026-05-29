from pathlib import Path

content = '''use age::x25519::Recipient;
use std::str::FromStr;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::io::Write;
use uuid::Uuid;

const UPLOAD_DIR: &str = "./vault";
const VAULT_PUB_KEY: &str = "age1y8pndue5j659v0cx3d2shj9v0m7n3pumxtuewm9cnyy8y48k2qmqrhlxew";

pub async fn trancar_no_cofre(dados: Vec<u8>) -> Result<(), ()> {
    let chave = Recipient::from_str(VAULT_PUB_KEY).map_err(|_| ())?;
    let nome_fantasma = Uuid::new_v4().to_string();
    let caminho_arquivo = format!("{}/{}.enc", UPLOAD_DIR, nome_fantasma);

    let encriptador = age::Encryptor::with_recipients(vec![Box::new(chave)]).map_err(|_| ())?;

    let mut dados_criptografados = vec![];

    {
        let mut funil = encriptador.wrap_output(&mut dados_criptografados).map_err(|_| ())?;
        funil.write_all(&dados).map_err(|_| ())?;
        funil.finish().map_err(|_| ())?;
    }

    let mut arquivo = File::create(&caminho_arquivo).await.map_err(|_| ())?;
    arquivo.write_all(&dados_criptografados).await.map_err(|_| ())?;

    Ok(())
}
'''

Path(r'C:\Users\nicol\Desktop\site\Satan\src\crypto.rs').write_text(content)
print('wrote', Path(r'C:\Users\nicol\Desktop\site\Satan\src\crypto.rs').stat().st_size)
 #esse script auxilia na gravação de conteudo no rust ele cria um string e escreve no arquivo de destino e fala o tamanho do arquivo gerado