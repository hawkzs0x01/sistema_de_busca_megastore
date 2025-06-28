use sistema_de_busca::gerador::gerar_produtos;

#[test]
fn testar_tamanho_do_gerador() {
    let produtos = gerar_produtos(100);
    assert_eq!(produtos.len(), 100);  // Espera-se que gere exatamente 100 produtos
}

#[test]
fn testar_ids_unicos() {
    let produtos = gerar_produtos(100);
    let mut ids = std::collections::HashSet::new();

    for produto in produtos {
        assert!(ids.insert(produto.id));  // Garante que o ID não seja repetido
    }
}

#[test]
fn testar_nomes_e_categorias() {
    let produtos = gerar_produtos(100);
    for produto in produtos {
        assert!(!produto.nome.is_empty());  // Nome não pode ser vazio
        assert!(!produto.categoria.is_empty());  // Categoria não pode ser vazia
    }
}

#[test]
fn testar_variedade_de_categorias() {
    let produtos = gerar_produtos(100);
    let categorias: std::collections::HashSet<_> = produtos.iter().map(|p| p.categoria.clone()).collect();
    assert!(categorias.len() > 1, "Deve haver mais de uma categoria gerada");
}