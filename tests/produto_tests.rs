use sistema_de_busca::produto::Produto;

#[test]
fn criar_produto() {
    let produto = Produto::new(1, "Notebook", "Eletrônicos", 2999.99);
    
    assert_eq!(produto.id, 1);
    assert_eq!(produto.nome, "Notebook");
    assert_eq!(produto.categoria, "Eletrônicos");
    assert_eq!(produto.preco, 2999.99);
}
