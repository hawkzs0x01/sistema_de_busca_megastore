mod produto;
mod index;
mod cache;
mod tree;
mod search;
mod persistencia;

use produto::Produto;
use index::Index;
use cache::Cache;
use tree::Tree;
use search::buscar;
use persistencia::{carregar_produtos, salvar_produtos, CAMINHO_PADRAO};

use std::io::{self, Write};

fn main() {
    let mut index = Index::new();
    let mut cache = Cache::new();
    let mut arvore = Tree::new();

    // Carrega produtos a partir do arquivo JSON
    let mut produtos = match carregar_produtos(CAMINHO_PADRAO) {
        Ok(lista) => lista,
        Err(e) => {
            eprintln!("Erro ao carregar produtos: {}", e);
            vec![]
        }
    };

    // Popular index e tree a partir dos produtos lidos
    for p in &produtos {
        index.add_product(p.clone());
        arvore.insert(p.clone());
    }

    println!("Bem-vindo ao Sistema de Busca de Produtos!");

    loop {
        println!("\nEscolha uma opção:");
        println!("1 - Buscar produtos por palavra-chave");
        println!("2 - Adicionar novo produto");
        println!("3 - Listar todos os produtos (Index)");
        println!("4 - Buscar produto por ID (Árvore)");
        println!("5 - Sair");

        print!("Opção: ");
        io::stdout().flush().unwrap();

        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).unwrap();
        let escolha = escolha.trim();

        match escolha {
            "1" => {
                print!("Digite a palavra-chave: ");
                io::stdout().flush().unwrap();
                let mut keyword = String::new();
                io::stdin().read_line(&mut keyword).unwrap();
                let keyword = keyword.trim();

                if let Some(produtos_encontrados) = buscar(&index, &mut cache, keyword) {
                    println!("Produtos encontrados:");
                    for p in produtos_encontrados {
                        println!("ID: {} | {} | {} | R${}", p.id, p.nome, p.categoria, p.preco);
                    }
                } else {
                    println!("Nenhum produto encontrado para '{}'.", keyword);
                }
            }
            "2" => {
                println!("Cadastro de novo produto:");
                print!("Nome: ");
                io::stdout().flush().unwrap();
                let mut nome = String::new();
                io::stdin().read_line(&mut nome).unwrap();
                let nome = nome.trim();

                print!("Categoria: ");
                io::stdout().flush().unwrap();
                let mut categoria = String::new();
                io::stdin().read_line(&mut categoria).unwrap();
                let categoria = categoria.trim();

                print!("Preço: ");
                io::stdout().flush().unwrap();
                let mut preco_str = String::new();
                io::stdin().read_line(&mut preco_str).unwrap();
                let preco: f32 = match preco_str.trim().replace(',', ".").parse() {
                    Ok(p) => p,
                    Err(_) => {
                        println!("Preço inválido!");
                        continue;
                    }
                };

                // Geração de novo ID
                let novo_id = produtos.iter().map(|p| p.id).max().unwrap_or(0) + 1;

                let produto = Produto::new(novo_id, nome, categoria, preco);
                produtos.push(produto.clone());
                index.add_product(produto.clone());
                arvore.insert(produto.clone());

                // Salva imediatamente após inserir
                if let Err(e) = salvar_produtos(CAMINHO_PADRAO, &produtos) {
                    eprintln!("Erro ao salvar produtos: {}", e);
                } else {
                    println!("Produto cadastrado com sucesso: {:?}", produto);
                }
            }
            "3" => {
                println!("Lista de todos os produtos no índice:");
                use std::collections::HashSet;
                let mut ids = HashSet::new();
                for produtos in index.index.values() {
                    for p in produtos {
                        if ids.insert(p.id) {
                            println!("ID: {} | {} | {} | R${}", p.id, p.nome, p.categoria, p.preco);
                        }
                    }
                }
            }
            "4" => {
                print!("Digite o ID do produto: ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                let id: u32 = match id_str.trim().parse() {
                    Ok(i) => i,
                    Err(_) => {
                        println!("ID inválido!");
                        continue;
                    }
                };

                match arvore.search(id) {
                    Some(prod) => {
                        println!("Produto encontrado: ID: {} | {} | {} | R${}", prod.id, prod.nome, prod.categoria, prod.preco);
                    }
                    None => {
                        println!("Produto com ID {} não encontrado.", id);
                    }
                }
            }
            "5" => {
                println!("Saindo. Obrigado por usar o sistema!");
                break;
            }
            _ => println!("Opção inválida. Tente novamente."),
        }
    }
}