use std::env;
use std::fs::File;
use std::io::prelude::*;
use sexp::*;
use sexp::Atom::*;

enum Expr {
    Num(i32),
    Add1(Box<Expr>),
    Sub1(Box<Expr>),
    Negate(Box<Expr>),
}

fn compile(program: String) -> String {
    let num = program.trim().parse::<i32>().unwrap();
    return format!("mov rax, {}", num);
}

fn eval(e: &Expr) -> i32 {
    match e {
        Expr::Num(n) => *n,
        Expr::Add1(n) => eval(n) + 1,
        Expr::Sub1(n) => eval(n) - 1,
        Expr::Negate(n) => - eval(n),
    }
}

fn parse_expr(s: &Sexp) -> Expr {

    match s {
        Sexp::Atom(I(n)) => Expr::Num(i32::try_from(*n).unwrap()),
        Sexp::List(vec) => {
            match &vec[..] {
                [Sexp::Atom(S(op)), e] if op == "add1" => Expr::Add1(Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), e] if op == "sub1" => Expr::Sub1(Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), e] if op == "negate" => Expr::Negate(Box::new(parse_expr(e))),
                _ => panic!("parse error"),
            }
        },
        _ => panic!("parse error"),
    }
}

fn compile_expr(e: &Expr) -> String {
    match e {
        Expr::Num(n) => format!("mov rax, {}", *n),
        Expr::Add1(subexpr) => compile_expr(subexpr) + "\nadd rax, 1",
        Expr::Sub1(subexpr) => compile_expr(subexpr) + "\nsub rax, 1",
        Expr::Negate(subexpr) => compile_expr(subexpr) + "\nneg rax",
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect(); // args() returns an iterator, .collect() changes the iterator to a collection like Vector or Hashmap

    let in_name = &args[1];
    let out_name = &args[2];

    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    let expr = parse_expr(&parse(&in_contents).unwrap());
    let result = compile_expr(&expr);

    let asm_program = format!("
    section .text
    global our_code_starts_here
    our_code_starts_here:
        {}
        ret
    ", result);

    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    // use the sexp parser
    let sexp = parse("(add1 (sub1 (add1 73)))").unwrap();
    println!("{:?}", sexp);

    Ok(()) // Rust auto-returns last expression
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        let expr1 = Expr::Num(10);
        let result = eval(&expr1);
        assert_eq!(result, 10);
    }

    #[test]
    fn test2() {
        let expr1 = Expr::Add1(Box::new(
            Expr::Sub1(
                Box::new(
                    Expr::Add1(Box::new(
                        Expr::Num(5)
                    ))
                )
            )
        ));

        let result = eval(&expr1);
        assert_eq!(result, 6);
    }
}