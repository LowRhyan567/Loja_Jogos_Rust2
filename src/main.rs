use std::io;
use mysql::prelude::*;
use mysql::*;

#[derive(Debug)]
struct Produto {
    pro_id: i32,
    pro_nome: String,
    prod_desc: String,
    prod_preco: f64,
    qntd_estoque: i32,
    cat_id: i32,
    forn_id: i32,
}

fn main() {
    unsafe { std::env::set_var("RUST_BACKTRACE", "1"); }
    let mut conn = conectar_banco();

    loop {
        println!("Escolha: 1. Listar 2. Adicionar 3. Atualizar 4. Remover 5. Sair");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: i32 = choice.trim().parse().unwrap_or(0);
        match choice {
            1 => listar_produtos(&mut conn),
            2 => adicionar_produto_cli(&mut conn),
            3 => atualizar_produto_cli(&mut conn),
            4 => remover_produto_cli(&mut conn),
            5 => break,
            _ => println!("Escolha inválida"),
        }
    }
}

fn conectar_banco() -> PooledConn {
    let url = "mysql://root:SENHA@localhost:3306/LUGAR"; // Credenciais
    let pool = Pool::new(url).expect("Falha ao criar pool");
    pool.get_conn().expect("Falha ao obter conexão")
}

fn listar_produtos(conn: &mut PooledConn) {
    let produtos: Vec<Produto> = conn
        .query_map(
            "SELECT pro_id, pro_nome, prod_desc, prod_preco, qntd_estoque, cat_id, forn_id FROM tb_produtos ORDER BY pro_nome",
            |(pro_id, pro_nome, prod_desc, prod_preco, qntd_estoque, cat_id, forn_id)| Produto { pro_id, pro_nome, prod_desc, prod_preco, qntd_estoque, cat_id, forn_id },
        )
        .unwrap_or_else(|e| {
            eprintln!("Erro ao consultar produtos: {}", e);
            Vec::new()
        });

    if produtos.is_empty() {
        println!("Nenhum produto encontrado.");
        return;
    }

    for p in produtos {
        println!("ID: {} | {} | R${:.2} | Estoque: {}\n{}\n", p.pro_id, p.pro_nome, p.prod_preco, p.qntd_estoque, p.prod_desc);
    }
}

fn adicionar_produto_db(conn: &mut PooledConn, nome: &str, desc: &str, preco: f64, estoque: i32) -> bool {
    let cat_id = 1;
    let forn_id = 1;
    conn.exec_drop(
        "INSERT INTO tb_produtos (pro_nome, prod_desc, prod_preco, qntd_estoque, cat_id, forn_id) VALUES (?, ?, ?, ?, ?, ?)",
        (nome, desc, preco, estoque, cat_id, forn_id),
    )
    .is_ok()
}

fn atualizar_produto_db(conn: &mut PooledConn, pro_id: i32, nome: &str, desc: &str, preco: f64, estoque: i32) -> bool {
    conn.exec_drop(
        "UPDATE tb_produtos SET pro_nome = ?, prod_desc = ?, prod_preco = ?, qntd_estoque = ? WHERE pro_id = ?",
        (nome, desc, preco, estoque, pro_id),
    )
    .is_ok()
}

fn remover_produto_db(conn: &mut PooledConn, pro_id: i32) -> bool {
    conn.exec_drop("DELETE FROM tb_produtos WHERE pro_id = ?", (pro_id,)).is_ok()
}

fn adicionar_produto_cli(conn: &mut PooledConn) {
    let mut nome = String::new();
    let mut desc = String::new();
    let mut preco = String::new();
    let mut estoque = String::new();

    println!("Nome:");
    io::stdin().read_line(&mut nome).unwrap();
    println!("Descrição:");
    io::stdin().read_line(&mut desc).unwrap();
    println!("Preço:");
    io::stdin().read_line(&mut preco).unwrap();
    println!("Quantidade em estoque:");
    io::stdin().read_line(&mut estoque).unwrap();

    let nome = nome.trim();
    let desc = desc.trim();
    let preco: f64 = preco.trim().parse().unwrap_or(0.0);
    let estoque: i32 = estoque.trim().parse().unwrap_or(0);

    if nome.is_empty() {
        println!("Nome obrigatório.");
        return;
    }

    if adicionar_produto_db(conn, nome, desc, preco, estoque) {
        println!("Jogo adicionado com sucesso!");
    } else {
        println!("Erro ao adicionar jogo.");
    }
}

fn atualizar_produto_cli(conn: &mut PooledConn) {
    println!("ID para atualizar:");
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    let id: i32 = id.trim().parse().unwrap_or(0);
    if id == 0 {
        println!("ID inválido.");
        return;
    }

    println!("Novo nome:");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).unwrap();
    println!("Nova descrição:");
    let mut desc = String::new();
    io::stdin().read_line(&mut desc).unwrap();
    println!("Novo preço:");
    let mut preco = String::new();
    io::stdin().read_line(&mut preco).unwrap();
    println!("Nova quantidade:");
    let mut estoque = String::new();
    io::stdin().read_line(&mut estoque).unwrap();

    let nome = nome.trim();
    let desc = desc.trim();
    let preco: f64 = preco.trim().parse().unwrap_or(0.0);
    let estoque: i32 = estoque.trim().parse().unwrap_or(0);

    if nome.is_empty() {
        println!("Nome obrigatório.");
        return;
    }

    if atualizar_produto_db(conn, id, nome, desc, preco, estoque) {
        println!("Jogo atualizado com sucesso!");
    } else {
        println!("Erro ao atualizar jogo.");
    }
}

fn remover_produto_cli(conn: &mut PooledConn) {
    println!("ID do produto para remover:");
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    let id: i32 = id.trim().parse().unwrap_or(0);
    if id == 0 {
        println!("ID inválido.");
        return;
    }

    if remover_produto_db(conn, id) {
        println!("Jogo removido com sucesso.");
    } else {
        println!("Erro ao remover jogo.");
    }
}
