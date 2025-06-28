use std::collections::HashMap;
use crate::produto::Produto;

pub struct Cache {
    store: HashMap<String, Vec<Produto>>,
}

#[allow(dead_code)]
impl Cache {
    /// Cria um cache vazio
    pub fn new() -> Cache {
        Cache {
            store: HashMap::new(),
        }
    }

    /// Adiciona produtos ao cache para uma palavra-chave específica
    pub fn add(&mut self, keyword: &str, produtos: &[Produto]) {
        self.store.insert(keyword.to_string(), produtos.to_vec());
    }

    /// Busca produtos no cache por palavra-chave
    pub fn get(&self, keyword: &str) -> Option<&Vec<Produto>> {
        self.store.get(keyword)
    }

    /// Retorna o número de entradas no cache
    pub fn len(&self) -> usize {
        self.store.len()
    }

    /// Limpa todas as entradas do cache
    pub fn clear(&mut self) {
        self.store.clear();
    }
}