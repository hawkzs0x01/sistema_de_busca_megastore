mod produto;
mod index;
mod cache;
mod tree;
mod search;
mod gerador;

use produto::Produto;
use index::Index;
use cache::Cache;
use tree::Tree;
use search::buscar;
use gerador::gerar_produtos;
use std::io::{self, Write};
use std::time::Instant;

fn main() {
    let mut produtos: Vec<Produto> = Vec::new();
    let mut index = Index::new();
    let mut cache = Cache::new();
    let mut arvore = Tree::new();

    println!("=== CLI DE TESTES MANUAIS ===");

    loop {
        println!();
        println!("Selecione uma opção:\n");
        println!("                 1 - Gerar produtos aleatórios
        ---------------------------------------------------------");
        println!("                 2 - Inserir produto manualmente
        ---------------------------------------------------------");
        println!("                 3 - Buscar por palavra-chave (com cache)
        ---------------------------------------------------------");
        println!("                 4 - Buscar por palavra-chave (sem cache)
        ---------------------------------------------------------");
        println!("                 5 - Buscar por ID na Árvore
        ---------------------------------------------------------");
        println!("                 6 - Limpar cache
        ---------------------------------------------------------");
        println!("                 7 - Limpar índice e árvore
        ---------------------------------------------------------");
        println!("                 8 - Listar todos os produtos
        ---------------------------------------------------------");
        println!("                 9 - Mostrar tamanho do cache/índice
        ---------------------------------------------------------");
        println!("                 10 - Simular buscas repetidas (benchmark cache)
        ---------------------------------------------------------");
        println!("                 11 - Sair
        ---------------------------------------------------------");

        print!("\nOpção: ");
        io::stdout().flush().unwrap();
        let mut opt = String::new();
        io::stdin().read_line(&mut opt).unwrap();
        let opt = opt.trim();

        match opt {
            "1" => {
                print!("Quantos produtos aleatórios gerar? ");
                io::stdout().flush().unwrap();
                let mut qtd_str = String::new();
                io::stdin().read_line(&mut qtd_str).unwrap();
                let qtd: usize = match qtd_str.trim().parse() {
                    Ok(v) => v,
                    Err(_) => { println!("Valor inválido."); continue; }
                };

                let novos = gerar_produtos(qtd);
                for p in &novos {
                    index.add_product(p.clone());
                    arvore.insert(p.clone());
                    produtos.push(p.clone());
                }
                println!("{} produtos gerados e adicionados.", qtd);
            }
            "2" => {
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

                let novo_id = produtos.iter().map(|p| p.id).max().unwrap_or(0) + 1;
                let produto = Produto::new(novo_id, nome, categoria, preco);
                index.add_product(produto.clone());
                arvore.insert(produto.clone());
                produtos.push(produto.clone());
                println!("Produto inserido: {:?}", produto);
            }
            "3" => {
                print!("Palavra-chave: ");
                io::stdout().flush().unwrap();
                let mut kw = String::new();
                io::stdin().read_line(&mut kw).unwrap();
                let kw = kw.trim();

                let start = Instant::now();
                let res = buscar(&index, &mut cache, kw);
                let dur = start.elapsed();

                match res {
                    Some(prods) => {
                        println!("{} produtos encontrados em {:?}", prods.len(), dur);
                        for p in prods.iter().take(5) {
                            println!("ID: {} | {} | {}", p.id, p.nome, p.categoria);
                        }
                        if prods.len() > 5 {
                            println!("... (total: {})", prods.len());
                        }
                    }
                    None => println!("Nenhum produto encontrado (tempo: {:?})", dur),
                }
            }
            "4" => {
                print!("Palavra-chave: ");
                io::stdout().flush().unwrap();
                let mut kw = String::new();
                io::stdin().read_line(&mut kw).unwrap();
                let kw = kw.trim();

                // Busca direto no índice, não usa cache
                let start = Instant::now();
                let res = index.search(kw);
                let dur = start.elapsed();

                match res {
                    Some(prods) => {
                        println!("{} produtos encontrados em {:?}", prods.len(), dur);
                        for p in prods.iter().take(5) {
                            println!("ID: {} | {} | {}", p.id, p.nome, p.categoria);
                        }
                        if prods.len() > 5 {
                            println!("... (total: {})", prods.len());
                        }
                    }
                    None => println!("Nenhum produto encontrado (tempo: {:?})", dur),
                }
            }
            "5" => {
                print!("ID do produto: ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                let id: u32 = match id_str.trim().parse() {
                    Ok(v) => v,
                    Err(_) => { println!("ID inválido."); continue; }
                };

                let start = Instant::now();
                match arvore.search(id) {
                    Some(p) => {
                        println!("Encontrado: {:?} (tempo: {:?})", p, start.elapsed());
                    }
                    None => println!("Produto não encontrado (tempo: {:?})", start.elapsed()),
                }
            }
            "6" => {
                cache.clear();
                println!("Cache limpo.");
            }
            "7" => {
                index = Index::new();
                arvore = Tree::new();
                produtos.clear();
                println!("Índice, árvore e produtos removidos.");
            }
            "8" => {
                println!("Lista de produtos:");
                for p in &produtos {
                    println!("ID: {} | {} | {} | R${}", p.id, p.nome, p.categoria, p.preco);
                }
                println!("Total: {}", produtos.len());
            }
            "9" => {
                let cache_size = cache.len();
                let index_size = index.index.len();
                println!("Cache: {} entradas", cache_size);
                println!("Index: {} chaves", index_size);
            }
            "10" => {
                print!("Palavra-chave para repetir busca: ");
                io::stdout().flush().unwrap();
                let mut kw = String::new();
                io::stdin().read_line(&mut kw).unwrap();
                let kw = kw.trim();

                print!("Quantas repetições? ");
                io::stdout().flush().unwrap();
                let mut reps_str = String::new();
                io::stdin().read_line(&mut reps_str).unwrap();
                let reps: usize = match reps_str.trim().parse() {
                    Ok(v) => v,
                    Err(_) => { println!("Valor inválido."); continue; }
                };

                cache = Cache::new(); // Garante cache vazio
                let mut total = 0;
                let start = Instant::now();
                for _ in 0..reps {
                    let res = buscar(&index, &mut cache, kw);
                    if let Some(prods) = res { total += prods.len(); }
                }
                let dur = start.elapsed();
                println!("{} buscas realizadas em {:?}", reps, dur);
                println!("Total de produtos retornados nas buscas: {}", total);
            }
            "11" => {
                println!("Saindo...");
                break;
            }
            _ => println!("Opção inválida!"),
        }
    }
}