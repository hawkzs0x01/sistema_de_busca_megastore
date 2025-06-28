use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)] 
#[derive(PartialEq)]  
pub struct Produto {
    pub id: u32,           // ID único do produto
    pub nome: String,      // Nome do produto
    pub categoria: String, // Categoria do produto
    pub preco: f32,        // Preço do produto
}

impl Produto {
    // Cria um novo produto
    pub fn new(id: u32, nome: &str, categoria: &str, preco: f32) -> Self {
        Self {
            id,
            nome: nome.to_string(),
            categoria: categoria.to_string(),
            preco,
        }
    }
}