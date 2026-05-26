use std::io;
use mysql::prelude::*;
use mysql::*;

#[derive(Debug)]
struct Categoria {
    cat_id: i32,
    cat_nome: String,
    cat_desc: String,
}

#[derive(Debug)]
struct Fornecedor {
    forn_id: i32,
    forn_nome: String,
    forn_num: String,
    forn_cnpj: String,
    forn_email: String,
    forn_ende: String,
}

#[derive(Debug)]
struct Produto {
    prod_id: i32,
    prod_nome: String,
    prod_desc: String,
    prod_preco: f64,
    quant_estoque: i32,
    categoria_id: i32,
    fornecedor_id: i32,
}

fn main() {

    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    let url = "mysql://root:SENHA@localhost:3306/NOME"; // Credenciais
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    loop {
        println!("Escolha a tabela: 1. Categoria 2. Fornecedor 3. Produto 4. Sair");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: i32 = choice.trim().parse().unwrap_or(0);
        match choice {
            1 => crud_categoria(&mut conn),
            2 => crud_fornecedor(&mut conn),
            3 => crud_produto(&mut conn),
            4 => break,
            _ => println!("Escolha inválida"),
        }
    }
}

fn crud_categoria(conn: &mut PooledConn) {
    loop {
        println!("CRUD Categoria: 1. Criar 2. Ler 3. Atualizar 4. Deletar 5. Voltar");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: i32 = choice.trim().parse().unwrap_or(0);
        match choice {
            1 => create_categoria(conn),
            2 => read_categorias(conn),
            3 => update_categoria(conn),
            4 => delete_categoria(conn),
            5 => break,
            _ => println!("Inválido"),
        }
    }
}

fn create_categoria(conn: &mut PooledConn) {
    println!("Nome:");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).unwrap();
    let nome = nome.trim();
    println!("Desc:");
    let mut desc = String::new();
    io::stdin().read_line(&mut desc).unwrap();
    let desc = desc.trim();
    conn.exec_drop("INSERT INTO tb_categoria (cat_nome, cat_desc) VALUES (?, ?)", (nome, desc)).unwrap();
    println!("Criado");
}

fn read_categorias(conn: &mut PooledConn) {
    let cats: Vec<Categoria> = conn.query_map("SELECT cat_id, cat_nome, cat_desc FROM tb_categoria", |(cat_id, cat_nome, cat_desc)| Categoria { cat_id, cat_nome, cat_desc }).unwrap();
    for cat in cats {
        println!("{:?}", cat);
    }
}

fn update_categoria(conn: &mut PooledConn) {
    println!("ID para atualizar:");
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    let id: i32 = id.trim().parse().unwrap();
    println!("Novo nome:");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).unwrap();
    let nome = nome.trim();
    println!("Nova desc:");
    let mut desc = String::new();
    io::stdin().read_line(&mut desc).unwrap();
    let desc = desc.trim();
    conn.exec_drop("UPDATE tb_categoria SET cat_nome = ?, cat_desc = ? WHERE cat_id = ?", (nome, desc, id)).unwrap();
    println!("Atualizado");
}

fn delete_categoria(conn: &mut PooledConn) {
    println!("ID para deletar:");
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    let id: i32 = id.trim().parse().unwrap();
    conn.exec_drop("DELETE FROM tb_categoria WHERE cat_id = ?", (id,)).unwrap();
    println!("Deletado");
}

fn crud_fornecedor(conn: &mut PooledConn) {
    loop {
        println!("CRUD Fornecedor: 1. Criar 2. Ler 3. Atualizar 4. Deletar 5. Voltar");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: i32 = choice.trim().parse().unwrap_or(0);
        match choice {
            1 => create_fornecedor(conn),
            2 => read_fornecedores(conn),
            3 => update_fornecedor(conn),
            4 => delete_fornecedor(conn),
            5 => break,
            _ => println!("Inválido"),
        }
    }
}

fn create_fornecedor(conn: &mut PooledConn) {
    println!("Nome:");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).unwrap();
    let nome = nome.trim();
    println!("Número:");
    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();
    let num = num.trim();
    println!("CNPJ:");
    let mut cnpj = String::new();
    io::stdin().read_line(&mut cnpj).unwrap();
    let cnpj = cnpj.trim();
    println!("Email:");
    let mut email = String::new();
    io::stdin().read_line(&mut email).unwrap();
    let email = email.trim();
    println!("Endereço:");
    let mut ende = String::new();
    io::stdin().read_line(&mut ende).unwrap();
    let ende = ende.trim();
    conn.exec_drop("INSERT INTO tb_fornecedor (forn_nome, forn_num, forn_cnpj, forn_email, forn_ende) VALUES (?, ?, ?, ?, ?)", (nome, num, cnpj, email, ende)).unwrap();
    println!("Criado");
}

fn read_fornecedores(conn: &mut PooledConn) {
    let forn: Vec<Fornecedor> = conn.query_map("SELECT forn_id, forn_nome, forn_num, forn_cnpj, forn_email, forn_ende FROM tb_fornecedor", |(forn_id, forn_nome, forn_num, forn_cnpj, forn_email, forn_ende)| Fornecedor { forn_id, forn_nome, forn_num, forn_cnpj, forn_email, forn_ende }).unwrap();
    for f in forn {
        println!("{:?}", f);
    }
}

fn update_fornecedor(conn: &mut PooledConn) {
    println!("ID para atualizar:");
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    let id: i32 = id.trim().parse().unwrap();
    println!("Novo nome:");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).unwrap();
    let nome = nome.trim();
    println!("Novo número:");
    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();
    let num = num.trim();
    println!("Novo CNPJ:");
    let mut cnpj = String::new();
    io::stdin().read_line(&mut cnpj).unwrap();
    let cnpj = cnpj.trim();
    println!("Novo email:");
    let mut email = String::new();
    io::stdin().read_line(&mut email).unwrap();
    let email = email.trim();
    println!("Novo endereço:");
    let mut ende = String::new();
    io::stdin().read_line(&mut ende).unwrap();
    let ende = ende.trim();
    conn.exec_drop("UPDATE tb_fornecedor SET forn_nome = ?, forn_num = ?, forn_cnpj = ?, forn_email = ?, forn_ende = ? WHERE forn_id = ?", (nome, num, cnpj, email, ende, id)).unwrap();
    println!("Atualizado");
}

fn delete_fornecedor(conn: &mut PooledConn) {
    println!("ID para deletar:");
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    let id: i32 = id.trim().parse().unwrap();
    conn.exec_drop("DELETE FROM tb_fornecedor WHERE forn_id = ?", (id,)).unwrap();
    println!("Deletado");
}

fn crud_produto(conn: &mut PooledConn) {
    loop {
        println!("CRUD Produto: 1. Criar 2. Ler 3. Atualizar 4. Deletar 5. Voltar");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: i32 = choice.trim().parse().unwrap_or(0);
        match choice {
            1 => create_produto(conn),
            2 => read_produtos(conn),
            3 => update_produto(conn),
            4 => delete_produto(conn),
            5 => break,
            _ => println!("Inválido"),
        }
    }
}

fn create_produto(conn: &mut PooledConn) {
    println!("Nome:");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).unwrap();
    let nome = nome.trim();
    println!("Desc:");
    let mut desc = String::new();
    io::stdin().read_line(&mut desc).unwrap();
    let desc = desc.trim();
    println!("Preço:");
    let mut preco = String::new();
    io::stdin().read_line(&mut preco).unwrap();
    let preco: f64 = preco.trim().parse().unwrap();
    println!("Quant Estoque:");
    let mut quant = String::new();
    io::stdin().read_line(&mut quant).unwrap();
    let quant: i32 = quant.trim().parse().unwrap();
    println!("Categoria ID:");
    let mut cat_id = String::new();
    io::stdin().read_line(&mut cat_id).unwrap();
    let cat_id: i32 = cat_id.trim().parse().unwrap();
    println!("Fornecedor ID:");
    let mut forn_id = String::new();
    io::stdin().read_line(&mut forn_id).unwrap();
    let forn_id: i32 = forn_id.trim().parse().unwrap();
    conn.exec_drop("INSERT INTO tb_produto (prod_nome, prod_desc, prod_preco, quant_estoque, categoria_id, fornecedor_id) VALUES (?, ?, ?, ?, ?, ?)", (nome, desc, preco, quant, cat_id, forn_id)).unwrap();
    println!("Criado");
}

fn read_produtos(conn: &mut PooledConn) {
    let prod: Vec<Produto> = conn.query_map("SELECT prod_id, prod_nome, prod_desc, prod_preco, quant_estoque, categoria_id, fornecedor_id FROM tb_produto", |(prod_id, prod_nome, prod_desc, prod_preco, quant_estoque, categoria_id, fornecedor_id)| Produto { prod_id, prod_nome, prod_desc, prod_preco, quant_estoque, categoria_id, fornecedor_id }).unwrap();
    for p in prod {
        println!("{:?}", p);
    }
}

fn update_produto(conn: &mut PooledConn) {
    println!("ID para atualizar:");
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    let id: i32 = id.trim().parse().unwrap();
    println!("Novo nome:");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).unwrap();
    let nome = nome.trim();
    println!("Nova desc:");
    let mut desc = String::new();
    io::stdin().read_line(&mut desc).unwrap();
    let desc = desc.trim();
    println!("Novo preço:");
    let mut preco = String::new();
    io::stdin().read_line(&mut preco).unwrap();
    let preco: f64 = preco.trim().parse().unwrap();
    println!("Nova quant:");
    let mut quant = String::new();
    io::stdin().read_line(&mut quant).unwrap();
    let quant: i32 = quant.trim().parse().unwrap();
    println!("Novo cat_id:");
    let mut cat_id = String::new();
    io::stdin().read_line(&mut cat_id).unwrap();
    let cat_id: i32 = cat_id.trim().parse().unwrap();
    println!("Novo forn_id:");
    let mut forn_id = String::new();
    io::stdin().read_line(&mut forn_id).unwrap();
    let forn_id: i32 = forn_id.trim().parse().unwrap();
    conn.exec_drop("UPDATE tb_produto SET prod_nome = ?, prod_desc = ?, prod_preco = ?, quant_estoque = ?, categoria_id = ?, fornecedor_id = ? WHERE prod_id = ?", (nome, desc, preco, quant, cat_id, forn_id, id)).unwrap();
    println!("Atualizado");
}

fn delete_produto(conn: &mut PooledConn) {
    println!("ID para deletar:");
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    let id: i32 = id.trim().parse().unwrap();
    conn.exec_drop("DELETE FROM tb_produto WHERE prod_id = ?", (id,)).unwrap();
    println!("Deletado");
}
