use sistema_de_busca::produto::Produto;
use sistema_de_busca::index::Index;
use std::fs;

#[test]
fn salvar_e_carregar_index_de_arquivo() {
    let mut index = Index::new();
    let produto = Produto::new(1, "Notebook", "Eletrônicos", 2999.99);
    index.add_product(produto.clone());

    // Serializa o index para JSON e salva em arquivo
    let filename = "test_index.json";
    let json = serde_json::to_string(&index.index).expect("Falha ao serializar index");
    fs::write(filename, &json).expect("Falha ao salvar arquivo");

    // Lê do arquivo e desserializa para um novo Index
    let json_lido = fs::read_to_string(filename).expect("Falha ao ler arquivo");
    let index_map: std::collections::HashMap<String, Vec<Produto>> =
        serde_json::from_str(&json_lido).expect("Falha ao desserializar index");
    let novo_index = Index { index: index_map };

    // Busca pelo produto no novo índice
    let resultado = novo_index.search("Notebook");
    assert!(resultado.is_some());
    let resultado = resultado.unwrap();
    assert_eq!(resultado[0].nome, "Notebook");

    // Limpa o arquivo de teste
    fs::remove_file(filename).unwrap();
}