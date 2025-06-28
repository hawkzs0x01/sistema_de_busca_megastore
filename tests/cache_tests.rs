use sistema_de_busca::cache::Cache;
use sistema_de_busca::produto::Produto;

#[test]
fn adicionar_e_recuperar_cache() {
    let mut cache = Cache::new();
    let produto = Produto::new(1, "Notebook", "Eletrônicos", 2999.99);
    cache.add("Eletrônicos", &[produto.clone()]);

    let produtos = cache.get("Eletrônicos");
    assert_eq!(produtos.as_ref().unwrap().len(), 1);
    assert_eq!(produtos.as_ref().unwrap()[0].nome, "Notebook");
}

#[test]
fn cache_vazio() {
    let cache = Cache::new();
    let produtos = cache.get("Inexistente");
    assert_eq!(produtos, None);
}

#[test]
fn limpar_cache() {
    let mut cache = Cache::new();
    let produto = Produto::new(2, "PC Gamer", "Computadores", 5999.99);
    cache.add("Computadores", &[produto]);
    assert_eq!(cache.len(), 1);
    cache.clear();
    assert_eq!(cache.len(), 0);
}

#[test]
fn sobrescrever_cache() {
    let mut cache = Cache::new();
    let produto1 = Produto::new(1, "Notebook", "Eletrônicos", 2999.99);
    let produto2 = Produto::new(2, "Tablet", "Eletrônicos", 1999.99);

    // Adiciona primeiro produto
    cache.add("Eletrônicos", &[produto1.clone()]);
    // Sobrescreve com outro produto
    cache.add("Eletrônicos", &[produto2.clone()]);

    let produtos = cache.get("Eletrônicos").unwrap();
    assert_eq!(produtos.len(), 1);
    assert_eq!(produtos[0].nome, "Tablet");
}

#[test]
fn multiplas_chaves() {
    let mut cache = Cache::new();
    let produto1 = Produto::new(1, "Notebook", "Eletrônicos", 2999.99);
    let produto2 = Produto::new(2, "Geladeira", "Eletrodomésticos", 1999.99);

    cache.add("Eletrônicos", &[produto1.clone()]);
    cache.add("Eletrodomésticos", &[produto2.clone()]);

    let eletronicos = cache.get("Eletrônicos").unwrap();
    let eletrodomesticos = cache.get("Eletrodomésticos").unwrap();

    assert_eq!(eletronicos.len(), 1);
    assert_eq!(eletronicos[0].nome, "Notebook");
    assert_eq!(eletrodomesticos.len(), 1);
    assert_eq!(eletrodomesticos[0].nome, "Geladeira");
}