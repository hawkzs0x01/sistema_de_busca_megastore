use std::collections::HashMap;
use crate::produto::Produto;

pub struct Index {
    pub index: HashMap<String, Vec<Produto>>,
}

impl Index {
    // Cria um novo índice vazio
    pub fn new() -> Self {
        Index {
            index: HashMap::new(),
        }
    }

    // Adiciona um produto ao índice, indexando por nome e categoria
    pub fn add_product(&mut self, produto: Produto) {
        let keywords = vec![produto.nome.clone(), produto.categoria.clone()];

        for keyword in keywords {
            self.index
                .entry(keyword)
                .or_insert_with(Vec::new)
                .push(produto.clone());
        }
    }

    // Busca produtos por uma palavra-chave
    pub fn search(&self, keyword: &str) -> Option<&Vec<Produto>> {
        self.index.get(keyword)
    }
}

