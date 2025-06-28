// src/gerador.rs

use crate::produto::Produto;

pub fn gerar_produtos(qtd: usize) -> Vec<Produto> {
    let mut produtos = Vec::with_capacity(qtd);

    for i in 0..qtd {
        let produto = Produto::new(
            i as u32,
            &format!("Produto {}", i),
            if i % 2 == 0 { "Categoria A" } else { "Categoria B" },
            (i as f32) * 1.5 + 10.0,
        );
        produtos.push(produto);
    }

    produtos
}