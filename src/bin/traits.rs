// Traits
// 1. Define a trait named Eval with a single method also called eval which returns an i64.
// 2. Implement the trait for the Expr data type.
// 3. Extend the main function to print the value of the evaluated expression.
trait Show {
    fn show(&self) -> String;
}

#[derive(PartialEq, Eq)]
enum Expr {
    Val(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

impl Show for Expr {
    fn show(&self) -> String {
        match self {
            Expr::Val(i) => i.to_string(),
            Expr::Add(l, r) => format!("({} + {})", l.show(), r.show()),
            Expr::Mul(l, r) => format!("({} * {})", l.show(), r.show()),
        }
    }
}

fn main() {
    // 3 * (1 + 2)
    let expr = Expr::Mul(
        Box::new(Expr::Val(3)),
        Box::new(Expr::Add(Box::new(Expr::Val(1)), Box::new(Expr::Val(2)))),
    );
    println!("{}", expr.show());
}
