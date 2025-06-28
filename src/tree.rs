use crate::produto::Produto;
use std::cell::RefCell;
use std::rc::Rc;

type NodeRef = Option<Rc<RefCell<TreeNode>>>;

pub struct TreeNode {
    pub produto: Produto,
    pub left: NodeRef,
    pub right: NodeRef,
}

pub struct Tree {
    pub root: NodeRef,
}

impl Tree {
    // Cria uma nova árvore vazia
    pub fn new() -> Self {
        Tree { root: None }
    }

    // Insere um produto na árvore, baseado no ID
    pub fn insert(&mut self, produto: Produto) {
        match self.root {
            None => {
                self.root = Some(Rc::new(RefCell::new(TreeNode {
                    produto,
                    left: None,
                    right: None,
                })));
            }
            Some(ref node) => Self::insert_node(node, produto),
        }
    }

    // Função auxiliar para inserir recursivamente, sobrescrevendo se o ID já existir
    fn insert_node(current: &Rc<RefCell<TreeNode>>, produto: Produto) {
        let mut current_borrowed = current.borrow_mut();
        let current_id = current_borrowed.produto.id;

        if produto.id < current_id {
            match current_borrowed.left {
                None => {
                    current_borrowed.left = Some(Rc::new(RefCell::new(TreeNode {
                        produto,
                        left: None,
                        right: None,
                    })));
                }
                Some(ref left) => Self::insert_node(left, produto),
            }
        } else if produto.id > current_id {
            match current_borrowed.right {
                None => {
                    current_borrowed.right = Some(Rc::new(RefCell::new(TreeNode {
                        produto,
                        left: None,
                        right: None,
                    })));
                }
                Some(ref right) => Self::insert_node(right, produto),
            }
        } else {
            // ID igual: sobrescreve o produto
            current_borrowed.produto = produto;
        }
    }

    // Busca um produto na árvore pelo ID
    pub fn search(&self, id: u32) -> Option<Produto> {
        Self::search_node(&self.root, id)
    }

    // Função auxiliar de busca
    fn search_node(node: &NodeRef, id: u32) -> Option<Produto> {
        match node {
            None => None,
            Some(n) => {
                let n_borrow = n.borrow();
                if id == n_borrow.produto.id {
                    Some(n_borrow.produto.clone())
                } else if id < n_borrow.produto.id {
                    Self::search_node(&n_borrow.left, id)
                } else {
                    Self::search_node(&n_borrow.right, id)
                }
            }
        }
    }
}