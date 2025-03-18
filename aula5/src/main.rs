// Implementação SUPER SIMPLIFICADA de Árvores Binárias em Rust
// Versão corrigida que compila sem erros

// =========================================
// ÁRVORE BINÁRIA DE BUSCA (BST) - BÁSICA
// =========================================

// Estrutura básica de um nó
#[derive(Debug)]
struct Node {
    value: i32,               // Valor do nó
    left: Option<Box<Node>>,  // Filho esquerdo
    right: Option<Box<Node>>, // Filho direito
}

// Árvore BST
#[derive(Debug)]
struct BST {
    root: Option<Box<Node>>, // Raiz da árvore
}

impl BST {
    // Criar árvore vazia
    fn new() -> Self {
        BST { root: None }
    }

    // Inserir valor
    fn insert(&mut self, value: i32) {
        // Usando uma abordagem recursiva sem múltiplos empréstimos mutáveis
        let mut current = &mut self.root;

        // Loop para encontrar o lugar certo para inserir
        loop {
            match current {
                // Lugar vazio encontrado, inserimos aqui
                None => {
                    *current = Some(Box::new(Node {
                        value,
                        left: None,
                        right: None,
                    }));
                    return;
                }
                // Navegamos para a posição correta
                Some(node) => {
                    if value < node.value {
                        // Ir para esquerda
                        current = &mut node.left;
                    } else if value > node.value {
                        // Ir para direita
                        current = &mut node.right;
                    } else {
                        // Valor duplicado, nada a fazer
                        return;
                    }
                }
            }
        }
    }

    // Buscar valor
    fn search(&self, value: i32) -> bool {
        let mut current = &self.root;

        // Loop para navegar até encontrar ou não o valor
        while let Some(node) = current {
            if value == node.value {
                return true; // Encontrou
            } else if value < node.value {
                current = &node.left; // Vai para esquerda
            } else {
                current = &node.right; // Vai para direita
            }
        }

        false // Não encontrou
    }
}

// =========================================
// ÁRVORE AVL - BÁSICA
// =========================================

// Estrutura básica de um nó AVL
#[derive(Debug, Clone)] // Adicionamos Clone para resolver o erro
struct AVLNode {
    value: i32,
    left: Option<Box<AVLNode>>,
    right: Option<Box<AVLNode>>,
    height: i32, // Altura para balanceamento
}

// Árvore AVL
#[derive(Debug)]
struct AVL {
    root: Option<Box<AVLNode>>,
}

impl AVL {
    // Criar árvore vazia
    fn new() -> Self {
        AVL { root: None }
    }

    // Altura de um nó
    fn height(node: &Option<Box<AVLNode>>) -> i32 {
        match node {
            None => 0,
            Some(n) => n.height,
        }
    }

    // Calcular balanceamento (diferença de altura entre subárvores)
    fn balance(node: &Option<Box<AVLNode>>) -> i32 {
        match node {
            None => 0,
            Some(n) => Self::height(&n.left) - Self::height(&n.right),
        }
    }

    // Rotação à direita (para caso esquerda-esquerda)
    fn rotate_right(mut y: Box<AVLNode>) -> Box<AVLNode> {
        let mut x = y.left.take().unwrap();
        y.left = x.right.take();

        // Atualizar alturas
        y.height = 1 + std::cmp::max(Self::height(&y.left), Self::height(&y.right));

        // Completar rotação
        x.right = Some(y);
        x.height = 1 + std::cmp::max(Self::height(&x.left), Self::height(&x.right));

        x
    }

    // Rotação à esquerda (para caso direita-direita)
    fn rotate_left(mut x: Box<AVLNode>) -> Box<AVLNode> {
        let mut y = x.right.take().unwrap();
        x.right = y.left.take();

        // Atualizar alturas
        x.height = 1 + std::cmp::max(Self::height(&x.left), Self::height(&x.right));

        // Completar rotação
        y.left = Some(x);
        y.height = 1 + std::cmp::max(Self::height(&y.left), Self::height(&y.right));

        y
    }

    // Inserir valor
    fn insert(&mut self, value: i32) {
        // Corrigido para evitar empréstimo duplo mutável
        let root = self.root.take();
        self.root = Self::insert_at(root, value);
    }

    // Método estático para inserção recursiva
    fn insert_at(node: Option<Box<AVLNode>>, value: i32) -> Option<Box<AVLNode>> {
        // Se o nó for None, criar novo nó
        if node.is_none() {
            return Some(Box::new(AVLNode {
                value,
                left: None,
                right: None,
                height: 1,
            }));
        }

        let mut node = node.unwrap();

        // Inserir como em BST normal
        if value < node.value {
            node.left = Self::insert_at(node.left.take(), value);
        } else if value > node.value {
            node.right = Self::insert_at(node.right.take(), value);
        } else {
            return Some(node); // Duplicata, retorna sem alterar
        }

        // Atualizar altura
        node.height = 1 + std::cmp::max(Self::height(&node.left), Self::height(&node.right));

        // Verificar balanceamento e fazer rotações
        let balance = Self::balance(&Some(node.clone()));

        // Caso esquerda-esquerda
        if balance > 1 && node.left.is_some() && value < node.left.as_ref().unwrap().value {
            return Some(Self::rotate_right(node));
        }

        // Caso direita-direita
        if balance < -1 && node.right.is_some() && value > node.right.as_ref().unwrap().value {
            return Some(Self::rotate_left(node));
        }

        // Caso esquerda-direita
        if balance > 1 && node.left.is_some() && value > node.left.as_ref().unwrap().value {
            let left = node.left.take().unwrap();
            node.left = Some(Self::rotate_left(left));
            return Some(Self::rotate_right(node));
        }

        // Caso direita-esquerda
        if balance < -1 && node.right.is_some() && value < node.right.as_ref().unwrap().value {
            let right = node.right.take().unwrap();
            node.right = Some(Self::rotate_right(right));
            return Some(Self::rotate_left(node));
        }

        Some(node)
    }
}

// Exemplo de uso
fn main() {
    println!("=== Árvore Binária de Busca ===");

    // BST exemplo
    let mut bst = BST::new();
    bst.insert(10);
    bst.insert(5);
    bst.insert(15);
    bst.insert(3);
    bst.insert(7);

    println!("10 existe na BST? {}", bst.search(10)); // true
    println!("20 existe na BST? {}", bst.search(20)); // false
    println!("Estrutura da BST: {:?}", bst);

    println!("\n=== Árvore AVL ===");

    // AVL exemplo
    let mut avl = AVL::new();
    // Insere valores que causariam desbalanceamento em BST
    avl.insert(10);
    avl.insert(20);
    avl.insert(30); // Causa rotação esquerda

    // Adicionar mais inserções para demonstrar o balanceamento
    avl.insert(40);
    avl.insert(50); // Mais rotações

    println!("Estrutura da AVL (balanceada): {:?}", avl);

    println!(
        "\nNote como a árvore AVL permanece balanceada mesmo com inserções em sequência crescente!"
    );
}
