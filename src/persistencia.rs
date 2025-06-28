use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter};
use crate::produto::Produto;

/// Caminho padrão do arquivo de persistência
pub const CAMINHO_PADRAO: &str = "produtos.json";

/// Carrega os produtos de um arquivo JSON.
/// Se o arquivo não existir, retorna um vetor vazio.
pub fn carregar_produtos(caminho: &str) -> io::Result<Vec<Produto>> {
    match File::open(caminho) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let produtos = serde_json::from_reader(reader).unwrap_or_else(|_| vec![]);
            Ok(produtos)
        },
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => Ok(vec![]),
        Err(e) => Err(e),
    }
}

/// Salva os produtos no arquivo JSON (sobrescreve).
pub fn salvar_produtos(caminho: &str, produtos: &[Produto]) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(caminho)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, produtos)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}