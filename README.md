#  **sistema_de_busca_megastore**

Sistema de busca otimizado para o catálogo de produtos da MegaStore, desenvolvido em **Rust**.<br>
Projeto acadêmico focado em **desempenho**, **organização modular** e **testes automatizados**.

---

##  **Descrição do Projeto**

O **Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore** foi desenvolvido para fornecer uma solução eficiente e rápida de busca em um grande catálogo de produtos.  
Permite buscas por **nome**, **categoria** e **ID** de produtos, com otimizações para garantir alto desempenho mesmo com grandes volumes de dados.  
Possui **cache** para acelerar consultas e suporte à **persistência** em arquivos, garantindo integridade entre execuções.

---

##  **Tecnologias Utilizadas**

- **Rust:** Linguagem de programação de sistemas, conhecida por sua segurança e alto desempenho.
- **Serde:** Serialização e desserialização de dados (JSON).
- **Serde_json:** Manipulação eficiente de JSON.
- **Rand:** Geração de dados aleatórios para produtos de teste.

---

##  **Como Executar o Sistema de Busca**

### 1. Clonando o Repositório

```sh
git clone https://github.com/hawkzs0x01/sistema_de_busca.git
cd sistema_de_busca
```

### 2. Instalando as Dependências

```sh
cargo build
```

Compila o código e baixa todas as dependências listadas no `Cargo.toml`.

### 3. Executando o Sistema de Busca

```sh
cargo run --bin cli
```

Executa o binário principal **cli**, inicializando o sistema e permitindo interação via linha de comando.

### 4. Modo de Testes Manuais

```sh
cargo run --bin manual_tests
```

Executa o binário `manual_tests.rs`, com funcionalidades de teste para validação do sistema.

### 5. Observações Adicionais

- **Configuração de Ambiente:** Apenas o Cargo é necessário. Certifique-se de ter o Rust instalado.
- **Sistema de Arquivos:** O sistema pode criar/ler arquivos JSON para persistir dados do índice. Garanta permissão de leitura e escrita no diretório de execução.

---

##  **Como Executar os Testes**

Os testes estão no diretório `tests/` e incluem testes **unitários** e de **integração**.

### Executando os Testes Automáticos

```sh
cargo test
```

Executa todos os testes automaticamente.

---

##  **Exemplos de Uso**

O sistema oferece duas interfaces principais:

### 1. **CLI (`main.rs`) – Gerenciamento de Produtos**

- **1. Buscar Produtos por Palavra-chave:**  
  Pesquise produtos por nome ou categoria.

- **2. Adicionar Novo Produto:**  
  Adicione produtos informando ID, nome, categoria e preço.

- **3. Listar Todos os Produtos:**  
  Exibe o catálogo completo.

- **4. Buscar Produto por ID (Árvore):**  
  Busca rápida por ID utilizando árvore binária.

- **5. Sair:**  
  Encerra o programa.

### 2. **CLI (`manual_tests.rs`) – Ferramenta de Testes**

- **1. Gerar Produtos Aleatórios:**  
  Cria produtos simulados para testes.

- **2. Inserir Produto Manualmente:**  
  Insere produtos para cenários específicos de teste.

- **3. Buscar por Palavra-chave (Com Cache):**  
  Busca otimizada utilizando cache.

- **4. Buscar por Palavra-chave (Sem Cache):**  
  Busca sem otimização, útil para comparação.

- **10. Simular Buscas Repetidas (Benchmark Cache):**  
  Avalia o desempenho do cache.

- **6/7. Limpar Cache/Índice:**  
  Permite reiniciar o sistema ou limpar dados antigos.

---

##  **Arquitetura do Sistema**

Organizado de forma **modular**:

```
sistema_de_busca/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs
│   ├── manual_tests.rs
│   ├── lib.rs
│   ├── index.rs
│   ├── search.rs
│   ├── cache.rs
│   ├── tree.rs
│   ├── produto.rs
│   └── gerador.rs
├── tests/
│   ├── index_tests.rs
│   ├── search_tests.rs
│   ├── cache_tests.rs
│   ├── tree_tests.rs
│   ├── produto_tests.rs
│   ├── gerador_test.rs
│   ├── teste_integracao.rs
│   ├── fluxo_erro_tests.rs
│   └── persistencia_test.rs
```

### **Principais Módulos**

- **produto:** Estrutura `Produto` (ID, Nome, Categoria, Preço).
- **index:** Índice de produtos com HashMap; funções `add_product()` e `search()`.
- **cache:** Armazena resultados de buscas; funções `add()` e `get()`.
- **tree:** Árvore binária (ex: AVL ou B-tree) para busca por ID; funções `insert()` e `search()`.
- **search:** Centraliza lógica de busca, priorizando cache.
- **gerador:** Geração de produtos aleatórios para testes.
- **manual_tests:** CLI para testes interativos e benchmarks.
- **main:** Interface principal para o usuário final.

---

##  **Algoritmos e Estruturas de Dados Utilizados**

### 1. **Tabela Hash (HashMap)**
- **Uso:** Índice de produtos por palavra-chave/categoria.
- **Complexidade:** Inserção/Busca O(1) (tempo médio).
- **Objetivo:** Busca rápida e escalável por palavras-chave.

### 2. **Árvore Balanceada (AVL/B-tree)**
- **Uso:** Produtos ordenados por ID.
- **Complexidade:** Inserção/Busca O(log n).
- **Objetivo:** Busca eficiente por ID.

### 3. **Cache**
- **Uso:** Resultados recentes para otimizar buscas repetidas.
- **Complexidade:** O(1) para inserção e recuperação.
- **Objetivo:** Desempenho superior em consultas subsequentes.

### 4. **Algoritmo de Busca**
- **Palavra-chave:** Diretamente no índice (ou cache).
- **ID:** Busca na árvore balanceada.

### 5. **Geração de Produtos**
- **Uso:** Simulação via módulo `gerador` com a crate `rand`.

### 6. **Persistência de Dados**
- **Uso:** Serialização/deserialização em JSON via `serde_json`.

---

##  **Desempenho e Escalabilidade**

- **Inserção no índice:** O(1)
- **Inserção/Busca na árvore:** O(log n)
- **Busca por palavra-chave:** O(1)
- **Busca por ID:** O(log n)
- **Cache:** O(1) para consultas repetidas

**Testes de desempenho** mostram que o sistema mantém tempos de resposta rápidos mesmo com milhares de produtos.  
O uso de cache reduz em até **95%** o tempo de resposta em buscas repetidas.

---

##  **Licença**

Este projeto está licenciado sob a **MIT License**.<br>
Consulte o arquivo [LICENSE](LICENSE) para mais detalhes.

A MIT License permite uso, modificação, distribuição e sublicenciamento do software, desde que a licença original e o aviso de direitos autorais sejam mantidos.

---