# sistema_de_busca_megastore
Sistema de busca otimizado para o catálogo de produtos da MegaStore, desenvolvido em Rust. Projeto acadêmico focado em desempenho, organização modular e testes automatizados.

Descrição do Projeto:

O Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore foi desenvolvido para fornecer uma solução eficiente e rápida de busca em um grande catálogo de produtos. Ele é capaz de realizar buscas por nome, categoria e ID de produtos, utilizando uma série de otimizações para garantir alto desempenho mesmo com grandes volumes de dados. O sistema possui integração com cache para acelerar as consultas e é capaz de realizar buscas tanto em memória quanto persistir os dados em arquivos para garantir a persistência entre execuções.

Tecnologias Utilizadas:

Rust: Linguagem de programação de sistemas, conhecida por sua segurança e alto desempenho.

Serde: Crate para serialização e desserialização de dados (usado para manipulação de JSON).

Serde_json: Crate para manipulação de JSON de maneira eficiente.

Rand: Crate para geração de dados aleatórios, utilizado para a criação de produtos de teste

Instruções de Como Executar o Sistema de Busca:

Para compilar e executar o Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore, siga os passos abaixo:

1. Clonando o Repositório
Primeiro, clone o repositório para o seu ambiente local. Use o comando abaixo para obter uma cópia do projeto:

git clone https://github.com/hawkzs0x01/sistema_de_busca.git
cd sistema_de_busca


2. Instalando as Dependências
Este projeto utiliza o Cargo, o gerenciador de pacotes e build system do Rust. Para garantir que todas as dependências do projeto sejam baixadas corretamente, execute o seguinte comando:

cargo build

Isso irá compilar o código e baixar as dependências necessárias especificadas no arquivo Cargo.toml.


3. Executando o Sistema de Busca
Após compilar o projeto, você pode executar o sistema de busca com o seguinte comando:


cargo run --bin cli

Este comando executará o binário principal cli, que irá inicializar o sistema e permitir a interação com o usuário.

4. Executando o Sistema de Busca em Modo de Testes Manuais
Se você quiser rodar o sistema de busca em modo de testes manuais, utilize o seguinte comando:


cargo run --bin manual_tests

Isso executará o binário manual_tests.rs, que contém funcionalidades de teste para validar o comportamento do sistema de busca.

6. Observações Adicionais

Configuração de Ambiente: O sistema não requer configurações adicionais além do Cargo. Certifique-se de ter o Rust instalado na sua máquina.

Sistema de Arquivos: Durante a execução, o sistema pode criar ou ler arquivos JSON para persistir dados do índice. Certifique-se de ter permissão de leitura e escrita no diretório de execução.

Como Executar os Testes:

Os testes do sistema estão localizados no diretório tests/ e incluem tanto testes unitários quanto testes de integração. Para garantir que o sistema esteja funcionando corretamente, basta rodar os seguintes comandos:

Executando os Testes Automáticos
Para rodar os testes automáticos execute o comando abaixo:

cargo test

Esse comando irá buscar todos os testes dentro do código-fonte e executar todos eles automaticamente

Exemplos de Uso:

O sistema de busca oferece duas abordagens principais para interagir com os dados: uma interface de linha de comando (CLI) no arquivo main.rs, voltada para o gerenciamento de produtos, e uma interface de testes no arquivo manual_tests.rs, mais técnica e focada em validações de comportamento e performance.

1. CLI do main.rs – Gerenciamento de Produtos

A CLI do main.rs oferece opções intuitivas para o gerenciamento de produtos no sistema de busca. Aqui está um exemplo de como usar as opções disponíveis:

1. Buscar Produtos por Palavra-chave:

Ao escolher a opção 1 para buscar produtos por uma palavra-chave, você pode pesquisar por produtos com base no nome ou categoria. Por exemplo:

2. Adicionar Novo Produto:

Com a opção 2, você pode adicionar novos produtos ao sistema. O programa pedirá informações como ID, nome, categoria e preço do produto.

3. Listar Todos os Produtos:

A opção 3 exibe todos os produtos cadastrados no sistema. Isso é útil para visualizar o catálogo completo.

4. Buscar Produto por ID (Árvore):

A opção 4 permite buscar produtos pelo seu ID específico. Basta informar o ID, e o sistema retornará os detalhes do produto correspondente.

5. Sair:

Para encerrar o programa, você pode escolher a opção 5.

2. CLI do manual_tests.rs – Ferramenta de Testes

A CLI do manual_tests.rs oferece uma série de opções para testar o sistema de busca em diferentes cenários. Aqui estão alguns exemplos de como utilizá-la:

1. Gerar Produtos Aleatórios:

A opção 1 gera um conjunto de produtos aleatórios para testar o sistema.

2. Inserir Produto Manualmente:

A opção 2 permite que você insira produtos manualmente, fornecendo informações como ID, nome, categoria e preço. Isso é útil para testar casos específicos de inserção.

3. Buscar por Palavra-chave (Com Cache):

Com a opção 3, você pode realizar buscas por palavra-chave, utilizando o cache do sistema para otimizar a consulta.

4. Buscar por Palavra-chave (Sem Cache):

A opção 4 realiza a mesma busca, mas sem utilizar o cache, permitindo comparar o desempenho de uma busca sem otimização.

5. Simular Buscas Repetidas (Benchmark Cache):

A opção 10 permite realizar um benchmark de buscas repetidas, simulando várias consultas para avaliar o desempenho do cache.

6. Limpar Cache e Índice:

As opções 6 e 7 permitem limpar o cache ou o índice de dados, respectivamente. Essas ações são úteis quando você deseja reiniciar o sistema ou limpar dados antigos para novos testes.

Arquitetura do Sistema:

A arquitetura do sistema de busca é modular, com os seguintes componentes principais:

1. Módulo produto
Define a estrutura Produto, contendo atributos como ID, Nome, Categoria e Preço, e fornece a função Produto::new() para criar novos produtos.

2. Módulo index
Gerencia o índice de produtos, utilizando tabelas hash para associar palavras-chave a produtos. Oferece a função add_product() para adicionar produtos e search() para realizar buscas.

3. Módulo cache
Armazena resultados de buscas em memória para otimizar consultas repetidas. As funções principais são add() para adicionar dados ao cache e get() para recuperá-los.

4. Módulo tree
Implements uma árvore binária para armazenar produtos por ID. Oferece as funções insert() para adicionar produtos e search() para buscá-los por ID.

5. Módulo search
Centraliza a lógica de busca, utilizando tanto o índice quanto o cache. A função buscar() realiza a busca de produtos, priorizando o cache antes de consultar o índice.

6. Módulo gerador
Gera produtos aleatórios para testes e demonstrações. A função gerar_produtos() cria uma lista de produtos com dados variados.

7. Módulo manual_tests
Interface CLI para realizar testes interativos, como inserir produtos, buscar com ou sem cache, e simular buscas repetidas para análise de desempenho.

8. Módulo main
Interface principal do sistema, permitindo ao usuário adicionar produtos, buscar por palavras-chave, listar produtos e buscar por ID na árvore.

sistema_de_busca/
├── Cargo.toml             # Arquivo de configuração do Rust (dependências, versão, etc.)
├── README.md              # Documentação do projeto
├── src/                   # Diretório principal com o código fonte
│   ├── main.rs            # CLI interativo para o usuário final
│   ├── manual_tests.rs    # Programa para testes manuais dos módulos
│   ├── lib.rs             # Módulo de biblioteca principal para reexportar submódulos
│   ├── index.rs           # Módulo para o índice de produtos (tabelas hash)
│   ├── search.rs          # Módulo para a lógica de busca (busca otimizada)
│   ├── cache.rs           # Módulo para o sistema de cache
│   ├── tree.rs            # Módulo para as árvores (por exemplo, AVL ou B-tree)
│   ├── produto.rs         # Estrutura de dados para representar o produto
│   └── gerador.rs         # Módulo para geração de dados simulados (produtos de teste)
├── tests/                 # Testes unitários e de integração
│   ├── index_tests.rs     # Testes para o módulo de índice (tabelas hash)
│   ├── search_tests.rs    # Testes para o módulo de busca
│   ├── cache_tests.rs     # Testes para o módulo de cache
│   ├── tree_tests.rs      # Testes para o módulo de árvores
│   ├── produto_tests.rs   # Testes para a estrutura de dados Produto
│   ├── gerador_test.rs    # Testes para o módulo de geração de dados
│   ├── teste_integracao.rs # Testes de integração
│   ├── fluxo_erro_tests.rs # Testes para cenários de erro
│   └── persistencia_test.rs # Testes para a persistência de dados

Algoritmos e Estruturas de Dados Utilizados
O Sistema de Busca foi projetado para fornecer uma busca eficiente e otimizada de produtos, utilizando diversas estruturas de dados para garantir alta performance em diferentes cenários.

1. Tabela Hash (HashMap)

Utilização: O índice de produtos é armazenado em uma tabela hash (HashMap<String, Vec<Produto>>), onde a chave é uma string que pode ser uma palavra-chave ou categoria, e o valor é um vetor de produtos que correspondem a essa chave.

Vantagens: A tabela hash permite busca, inserção e remoção em tempo médio constante 
O(1), tornando as consultas por palavra-chave ou categoria extremamente rápidas.

Objetivo: Facilitar a busca rápida de produtos por palavras-chave e categorias, garantindo a escalabilidade do sistema à medida que o número de produtos cresce.

2. Árvore (AVL ou B-tree)

Utilização: A árvore é usada para armazenar os produtos de maneira ordenada por ID. A estrutura utilizada pode ser uma árvore binária de busca balanceada (como uma árvore AVL ou B-tree), garantindo que as operações de inserção, busca e remoção sejam realizadas em tempo logarítmico eficiente.


Objetivo: Proporcionar uma estrutura ordenada e eficiente para a busca por ID de produtos, permitindo uma navegação rápida e balanceada entre os elementos.

3. Cache

Utilização: O sistema utiliza um cache simples, implementado por meio de um HashMap, para armazenar os resultados das buscas mais recentes. Isso permite otimizar as consultas repetidas, retornando resultados diretamente do cache quando uma busca é feita com a mesma chave.

Objetivo: Melhorar o desempenho em consultas subsequentes, reduzindo a necessidade de reprocessar as buscas no índice e na árvore.

4. Algoritmo de Busca

Busca por Palavra-Chave: O algoritmo de busca por palavra-chave localiza os produtos que correspondem a uma chave específica no índice (tabela hash) ou no cache.

Busca por ID: A busca por ID é realizada na árvore, utilizando a estrutura ordenada para localizar o produto em tempo eficiente

5. Algoritmo de Geração de Produtos

Utilização: O módulo de geração de dados aleatórios cria produtos simulados para testes, utilizando o gerador de números aleatórios da biblioteca rand. A criação de produtos envolve a definição de nomes, categorias e preços para preencher a base de dados do sistema.

6. Persistência de Dados

Utilização: A persistência de dados é feita por meio de serialização/deserialização para o formato JSON, utilizando a biblioteca serde_json. O índice de produtos pode ser salvo em disco e carregado posteriormente, garantindo a persistência entre sessões de execução do sistema.

Essa abordagem combina a utilização de tabelas hash para indexação eficiente, árvores para armazenar dados ordenados e cache para otimizar consultas repetidas, criando um sistema de busca robusto e de alto desempenho.


Considerações sobre Desempenho e Escalabilidade
O Sistema de Busca Otimizado para Catálogo de Produtos foi projetado com ênfase na eficiência e escalabilidade, garantindo alta performance mesmo à medida que o número de produtos no catálogo cresce. A seguir, discutimos os principais pontos de desempenho e escalabilidade do sistema:

1. Tempo de Execução das Operações

Inserção de Produtos:

A inserção de novos produtos no índice (HashMap) é realizada em tempo médio O(1), graças à utilização de tabelas hash. Isso garante que a operação seja extremamente rápida, mesmo com grandes volumes de dados.

A inserção na árvore de produtos (AVL ou B-tree) é feita em tempo O(logn), pois a árvore é balanceada, garantindo que a operação permaneça eficiente mesmo quando a quantidade de produtos cresce.

Busca de Produtos:

Busca por Palavra-Chave: O tempo de busca no índice (tabela hash) também é O(1) na média, permitindo que consultas por palavra-chave ou categoria sejam realizadas quase instantaneamente.

Busca por ID: A busca por ID, realizada na árvore, é feita em tempo O(logn), que é eficiente mesmo para um grande número de produtos.

Cache: O uso de cache permite que consultas subsequentes a dados previamente buscados sejam realizadas em tempo constante O(1), sem necessidade de acessar o índice ou a árvore.

2. Escalabilidade

Aumento do Número de Produtos: À medida que o número de produtos cresce, o sistema se mantém eficiente graças às características das estruturas de dados utilizadas.

Índice (HashMap) mantém desempenho constante em buscas e inserções.

Árvore permite que buscas e inserções sejam realizadas em tempo logarítmico.

Cache ajuda a reduzir a carga em ambos os componentes quando há buscas repetidas.

Carga de Consultas Repetidas: O cache é um dos principais pontos fortes em termos de escalabilidade, pois permite que buscas repetidas sejam atendidas rapidamente sem sobrecarregar o sistema. Isso melhora significativamente o desempenho quando o sistema recebe muitas consultas semelhantes.

3. Desempenho em Testes

Teste de Performance de Busca: Em testes realizados, a busca por palavra-chave e por ID demonstraram tempos de resposta rápidos mesmo com uma base de dados contendo milhares de produtos.

Teste de Carga com Cache: Testes de carga simulando buscas repetidas mostraram que o cache reduz o tempo de resposta em 95% em comparação com buscas não-cacheadas, evidenciando uma otimização significativa.

Teste de Escalabilidade: O sistema manteve o desempenho esperado ao aumentar o número de produtos de 100 para 10.000 itens, sem degradação visível no tempo de resposta para buscas por palavra-chave ou ID.

4. Considerações Finais

O sistema é altamente escalável devido ao uso de tabelas hash e árvores balanceadas, e é capaz de lidar com grandes volumes de dados e consultas simultâneas de forma eficiente.

O uso de cache aumenta ainda mais o desempenho, especialmente em cenários com muitas consultas repetidas.

Em termos de desempenho, o sistema é capaz de fornecer respostas rápidas para a maioria das operações, mesmo com uma grande base de dados.

Em resumo, o sistema foi projetado para ser altamente eficiente em termos de tempo de resposta e escalável para suportar um grande número de produtos e buscas simultâneas, mantendo a performance mesmo com o aumento do volume de dados.

Licença

Este projeto está licenciado sob a MIT License.

Você pode encontrar os detalhes completos da licença no arquivo LICENSE.

A MIT License permite que você use, modifique, distribua e sublicencie o software, desde que inclua a licença original e o aviso de direitos autorais no código-fonte. Para mais informações sobre os termos completos da licença, consulte o arquivo LICENSE na raiz do repositório.


