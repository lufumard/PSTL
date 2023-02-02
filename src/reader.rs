use chumsky::Parser;
use chumsky::prelude::*;

mod ast;


fn parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    
    // Fonction définition d'un opérateur
    let op = |c| just(c).padded();

    // Définition d'un identifiant (texte)
    let ident = text::ident()
    .padded();

    let expr = recursive(|expr| {
        let int = text::int(10)
            .map(|s: String| Expr::Num(s.parse().unwrap()))
            .padded();

        let atom = int
            .or(expr.delimited_by(just('('), just(')'))).padded();

        let r#let = text::keyword("let")
            .delimited_by(just('('), just(')'))
            .ignore_then(ident) // Name
            .then(expr.clone()) // Value
            .then(expr.clone()) // Body
            .map(|((name, rhs), then)| Expr::Let {
                name,
                rhs: Box::new(rhs),
                then: Box::new(then),
            });
        
        let r#fun = text::keyword("fun")
            .delimited_by(just('('), just(')'))
            .ignore_then(ident) // Name
            .then(ident.clone().repeated()) // Parameters
            .then(expr.clone()) // Body
            .map(|((name, args), body)| Expr::Fun {
                name,
                args,
                body: Box::new(body),
            });
        
        let r#fun = text::keyword("if")
            .delimited_by(just('('), just(')'))
            .ignore_then(expr.clone()) // Cond
            .then(expr.clone().repeated()) // then
            .then(expr.clone()) // elze
            .map(|((cond, then), elze)| Expr::Fun {
                cond: Box::new(cond),
                then: Box::new(then),
                elze: Box::new(elze),
            });

    });
}