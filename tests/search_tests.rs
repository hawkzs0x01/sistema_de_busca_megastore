use sistema_de_busca::produto::Produto;
use sistema_de_busca::index::Index;
use sistema_de_busca::cache::Cache;
use sistema_de_busca::search::buscar;

#[test]
#[allow(unused_mut)]
fn buscar_produto_no_indice_e_atualizar_cache() {
    let mut index = Index::new();
    let mut cache = Cache::new();
    let produto = Produto::new(1, "Notebook", "Eletrônicos", 2999.99);
    index.add_product(produto.clone());

    let resultado = buscar(&index, &mut cache, "Notebook");
    assert!(resultado.is_some());
    let resultado = resultado.unwrap();
    assert_eq!(resultado.len(), 1);
    assert_eq!(resultado[0].nome, "Notebook");

    let cache_result = cache.get("Notebook");
    assert!(cache_result.is_some());
    let cache_result = cache_result.unwrap();
    assert_eq!(cache_result[0].nome, "Notebook");
}

#[test]
#[allow(unused_mut)]
fn buscar_produto_no_cache() {
    let mut index = Index::new();
    let mut cache = Cache::new();
    let produto = Produto::new(1, "Notebook", "Eletrônicos", 2999.99);
    cache.add("Notebook", &[produto.clone()]);

    let resultado = buscar(&index, &mut cache, "Notebook");
    assert!(resultado.is_some());
    let resultado = resultado.unwrap();
    assert_eq!(resultado.len(), 1);
    assert_eq!(resultado[0].nome, "Notebook");
}

#[test]
#[allow(unused_mut)]
fn buscar_termo_inexistente() {
    let mut index = Index::new();
    let mut cache = Cache::new();
    let resultado = buscar(&index, &mut cache, "Inexistente");
    assert!(resultado.is_none());
    assert!(cache.get("Inexistente").is_none());
}

#[test]
#[allow(unused_mut)]
fn buscar_multiplos_produtos_mesma_chave() {
    let mut index = Index::new();
    let mut cache = Cache::new();
    let produto1 = Produto::new(2, "Notebook", "Eletrônicos", 2999.99);
    let produto2 = Produto::new(3, "Tablet", "Eletrônicos", 1999.99);
    index.add_product(produto1.clone());
    index.add_product(produto2.clone());

    let resultado = buscar(&index, &mut cache, "Eletrônicos");
    assert!(resultado.is_some());
    let resultado = resultado.unwrap();
    assert_eq!(resultado.len(), 2);
    let nomes: Vec<_> = resultado.iter().map(|p| p.nome.as_str()).collect();
    assert!(nomes.contains(&"Notebook"));
    assert!(nomes.contains(&"Tablet"));
}

#[test]
#[allow(unused_mut)]
fn buscar_apos_limpar_cache() {
    let mut index = Index::new();
    let mut cache = Cache::new();
    let produto = Produto::new(4, "PC Gamer", "Computadores", 5999.99);
    index.add_product(produto.clone());

    let _ = buscar(&index, &mut cache, "Computadores");
    cache.clear();
    let resultado = buscar(&index, &mut cache, "Computadores");
    assert!(resultado.is_some());
    let resultado = resultado.unwrap();
    assert_eq!(resultado.len(), 1);
    assert_eq!(resultado[0].nome, "PC Gamer");
}