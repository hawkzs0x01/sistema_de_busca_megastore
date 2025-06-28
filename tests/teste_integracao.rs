use sistema_de_busca::produto::Produto;
use sistema_de_busca::index::Index;
use sistema_de_busca::cache::Cache;
use std::fs;

#[test]
fn fluxo_completo_index_persistencia_cache() {
    // Cria índice e cache
    let mut index = Index::new();
    let mut cache = Cache::new();

    // Adiciona produtos
    let notebook = Produto::new(1, "Notebook", "Eletrônicos", 2999.99);
    let camiseta = Produto::new(2, "Camiseta", "Vestuário", 49.90);
    index.add_product(notebook.clone());
    index.add_product(camiseta.clone());

    // Salva o índice em disco
    let filename = "integration_index.json";
    let json = serde_json::to_string(&index.index).expect("Falha ao serializar index");
    fs::write(filename, &json).expect("Falha ao salvar arquivo");

    // Limpa estruturas da memória
    let mut index = Index::new();
    cache.clear();

    // Carrega o índice do disco
    let json_lido = fs::read_to_string(filename).expect("Falha ao ler arquivo");
    let index_map: std::collections::HashMap<String, Vec<Produto>> =
        serde_json::from_str(&json_lido).expect("Falha ao desserializar index");
    index.index = index_map;

    // Busca produto no índice carregado
    let resultado = index.search("Notebook");
    assert!(resultado.is_some());
    let resultado = resultado.unwrap();
    assert_eq!(resultado[0].nome, "Notebook");

    // Testa integração com cache: adiciona resultado no cache e busca lá
    cache.add("Notebook", resultado);
    let cache_result = cache.get("Notebook");
    assert!(cache_result.is_some());
    let cache_result = cache_result.unwrap();
    assert_eq!(cache_result[0].nome, "Notebook");

    // Remove arquivo de teste
    fs::remove_file(filename).unwrap();
}