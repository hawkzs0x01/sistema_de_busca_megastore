use sistema_de_busca::produto::Produto;
use sistema_de_busca::tree::Tree;

#[test]
fn inserir_e_buscar_na_arvore() {
    let mut arvore = Tree::new();

    let produto1 = Produto::new(1, "Notebook", "Eletrônicos", 2999.99);
    let produto2 = Produto::new(2, "Camiseta", "Vestuário", 49.90);
    let produto3 = Produto::new(3, "Smartphone", "Eletrônicos", 1999.99);

    arvore.insert(produto1.clone());
    arvore.insert(produto2.clone());
    arvore.insert(produto3.clone());

    // Testa busca dos 3 produtos
    let res1 = arvore.search(1).unwrap();
    assert_eq!(res1.nome, "Notebook");
    assert_eq!(res1.preco, 2999.99);

    let res2 = arvore.search(2).unwrap();
    assert_eq!(res2.nome, "Camiseta");
    assert_eq!(res2.preco, 49.90);

    let res3 = arvore.search(3).unwrap();
    assert_eq!(res3.nome, "Smartphone");
    assert_eq!(res3.preco, 1999.99);
}

#[test]
fn buscar_produto_na_arvore_inexistente() {
    let mut arvore = Tree::new();

    let produto1 = Produto::new(1, "Notebook", "Eletrônicos", 2999.99);
    arvore.insert(produto1.clone());

    let resultado = arvore.search(999);  // ID inexistente
    assert_eq!(resultado, None);
}

#[test]
fn buscar_em_arvore_vazia() {
    let arvore = Tree::new();
    assert_eq!(arvore.search(1), None);
}

#[test]
fn inserir_produto_id_duplicado() {
    let mut arvore = Tree::new();

    let produto1 = Produto::new(1, "Notebook", "Eletrônicos", 2999.99);
    let produto2 = Produto::new(1, "Notebook Gamer", "Eletrônicos", 4999.99);

    arvore.insert(produto1.clone());
    arvore.insert(produto2.clone());

    // Dependendo da sua lógica de inserção, pode manter o primeiro, substituir, ou ignorar;
    // Ajuste essa verificação conforme sua implementação!
    let resultado = arvore.search(1).unwrap();
    // Aqui supondo que a árvore mantém o último inserido:
    assert_eq!(resultado.nome, "Notebook Gamer");
}