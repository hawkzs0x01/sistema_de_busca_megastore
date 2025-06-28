use sistema_de_busca::index::Index;
use sistema_de_busca::produto::Produto;

#[test]
fn adicionar_e_buscar_produto() {
    let mut index = Index::new();
    let produto = Produto::new(1, "Notebook", "Eletrônicos", 2999.99);
    index.add_product(produto.clone());

    let produtos = index.search("Notebook");
    assert_eq!(produtos.as_ref().unwrap().len(), 1);
    assert_eq!(produtos.as_ref().unwrap()[0].nome, "Notebook");
}

#[test]
fn buscar_categoria_inexistente() {
    let mut index = Index::new();
    let produto = Produto::new(1, "Notebook", "Eletrônicos", 2999.99);
    index.add_product(produto.clone());

    let produtos = index.search("Vestuário");
    assert_eq!(produtos, None);
}

#[test]
fn buscar_por_categoria_existente() {
    let mut index = Index::new();
    let produto = Produto::new(2, "Geladeira", "Eletrodomésticos", 1999.99);
    index.add_product(produto.clone());

    let produtos = index.search("Eletrodomésticos");
    assert_eq!(produtos.as_ref().unwrap().len(), 1);
    assert_eq!(produtos.as_ref().unwrap()[0].nome, "Geladeira");
}

#[test]
fn buscar_multiplos_produtos_mesma_palavra() {
    let mut index = Index::new();
    let produto1 = Produto::new(3, "Camiseta", "Vestuário", 49.99);
    let produto2 = Produto::new(4, "Calça", "Vestuário", 89.99);

    index.add_product(produto1.clone());
    index.add_product(produto2.clone());

    let produtos = index.search("Vestuário").unwrap();
    let nomes: Vec<_> = produtos.iter().map(|p| p.nome.as_str()).collect();

    assert!(nomes.contains(&"Camiseta"));
    assert!(nomes.contains(&"Calça"));
    assert_eq!(produtos.len(), 2);
}