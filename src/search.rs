use crate::cache::Cache;
use crate::index::Index;
use crate::produto::Produto;

pub fn buscar(
    index: &Index,
    cache: &mut Cache,
    keyword: &str,
) -> Option<Vec<Produto>> {
    if let Some(result) = cache.get(keyword) {
        println!("(Cache) Resultado encontrado para '{}'", keyword);
        return Some(result.clone());
    }

    if let Some(result) = index.search(keyword) {
        println!("(Índice) Resultado encontrado para '{}'", keyword);
        cache.add(keyword, result);
        return Some(result.clone());
    }

    println!("Nenhum resultado encontrado para '{}'", keyword);
    None
}