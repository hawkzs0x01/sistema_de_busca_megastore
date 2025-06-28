use sistema_de_busca::produto::Produto;
use sistema_de_busca::index::Index;
use std::fs;

#[test]
fn buscar_palavra_inexistente_no_index() {
    let mut index = Index::new();
    let produto = Produto::new(1, "Notebook", "Eletrônicos", 2999.99);
    index.add_product(produto);

    // Busca por palavra que não existe
    let resultado = index.search("Celular");
    assert!(resultado.is_none());
}

#[test]
fn buscar_em_index_vazio() {
    let index = Index::new();
    let resultado = index.search("Qualquer");
    assert!(resultado.is_none());
}

#[test]
fn carregar_index_de_arquivo_inexistente() {
    // Tenta ler um arquivo que não existe
    let filename = "arquivo_que_nao_existe.json";
    let result = fs::read_to_string(filename);
    assert!(result.is_err());
}

#[test]
fn carregar_index_de_arquivo_corrompido() {
    // Cria um arquivo inválido temporariamente
    let filename = "corrompido.json";
    fs::write(filename, "isso_nao_eh_json").unwrap();

    let json = fs::read_to_string(filename).unwrap();
    let result: Result<std::collections::HashMap<String, Vec<Produto>>, _> =
        serde_json::from_str(&json);
    assert!(result.is_err());

    fs::remove_file(filename).unwrap();
}