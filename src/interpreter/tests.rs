#[cfg(test)]
use std::collections::HashMap;
use std::fs;

use chumsky::Parser;

use crate::ast::{CONST_LIST, FnBody};

use crate::interpreter::*;
use crate::reader;

use primitives::extract_int;

fn get_num(l:Loc, h:&Heap) -> i32 {
    return extract_int(l, h);
}

fn get_ctor(l:Loc, h:&Heap) -> (i32, Vec<Loc>) {
    if let (Value::Ctor(v, a), _) = h.get(l){
        return (v, a);
    }else {
        panic!("Pas un ctor");
    }
}

fn get_pap(l:Loc, h:&Heap) -> (Const, Vec<Loc>) {
    if let (Value::Pap(c, a), _) = h.get(l){
        return (c, a);
    }else {
        panic!("Pas un ctor");
    }
}

fn get_bool(l:Loc, h:&Heap) -> bool {
    if let (Value::Ctor(n, _), _) = h.get(l){
        match n {
            CONST_FALSE => false,
            CONST_TRUE => true,
            _ => panic!("{} pas constructeur de bool", n)
        }
    }else{
        panic!("Pas un bool");
    }
}

fn add_value(var:Var, val:Value, c:Ctxt, h:&mut Heap) -> Ctxt {
    c.add(var, h.add((val, 1)))
}

#[test]
fn test_ret() {
    let mut heap = empty_heap();
    let mut ctxt = empty_ctxt();
    let mut lfn = HashMap::new();

    let var = Var::Var("var".to_string());
    let value = make_num(0);

    let l = heap.add((value, 1));
    ctxt = ctxt.add(var.clone(), l.clone());

    let res = eval_ret(var, &ctxt, &mut heap, &mut lfn);
    assert_eq!(l, res);
}

#[test]
fn test_let() {
    let mut heap = empty_heap();
    let mut ctxt = empty_ctxt();
    let mut lfn = HashMap::new();

    let var = Var::Var("var".to_string());
    let expected = 5;
    let value = Expr::Num(expected.clone());
    let fnbody = FnBody::Ret(var.clone());

    let res = eval_let(var, value, fnbody, &mut ctxt, &mut heap, &mut lfn);
    let n = get_num(res, &heap);
    assert_eq!(expected, n);
}

#[test]
fn test_ctor() {
    let mut heap = empty_heap();
    let mut ctxt = empty_ctxt();
    let mut lfn = HashMap::new();

    let l1 = heap.add((make_num(1), 1));
    let l2 = heap.add((make_num(2), 1));
    ctxt = ctxt.add(Var::Var("a".to_string()), l1.clone());
    ctxt = ctxt.add(Var::Var("b".to_string()), l2.clone());
    let expected_args = vec![Var::Var("a".to_string()), Var::Var("b".to_string())];

    let res = eval_ctor(CONST_LIST, expected_args.clone(), &ctxt, &mut heap, &mut lfn);
    let (n, a) = get_ctor(res, &heap);
    
    assert_eq!(CONST_LIST, n);
    
    assert_eq!(l1, a[0]);
    assert_eq!(l2, a[1]);
}

#[test]
fn test_proj1() {
    let mut heap = empty_heap();
    let mut ctxt = empty_ctxt();
    let mut lfn = HashMap::new();

    let var = Var::Var("var".to_string());
    let l1 = heap.add((make_num(1), 1));
    let l2 = heap.add((make_num(2), 1));

    let args = vec![l1.clone(), l2.clone()];
    let ctor = Value::Ctor(CONST_LIST, args);
    
    let l = heap.add((ctor, 1));
    ctxt = ctxt.add(var.clone(), l.clone());

    let res = eval_proj(1, var, &ctxt, &mut heap, &mut lfn);
    assert_eq!(l1, res);
}

#[test]
fn test_case_false() {
    let mut heap = empty_heap();
    let mut ctxt = empty_ctxt();
    let mut lfn = HashMap::new();

    let v1 = Var::Var("v1".to_string());
    let l1 = heap.add((make_num(1), 1));
    let v2 = Var::Var("v2".to_string());
    let l2 = heap.add((make_num(2), 1));

    let cases = vec![FnBody::Ret(v1.clone()), FnBody::Ret(v2.clone())];
    ctxt = ctxt.add(v1.clone(), l1.clone());
    ctxt = ctxt.add(v2.clone(), l2.clone());

    let var =  Var::Var("var".to_string());
    let l_var = heap.add((make_false(), 1));
    ctxt = ctxt.add(var.clone(), l_var.clone());

    let res = eval_case(var.clone(), cases, &ctxt, &mut heap, &mut lfn);
    assert_eq!(l1, res);
}

#[test]
fn test_case_true() {
    let mut heap = empty_heap();
    let mut ctxt = empty_ctxt();
    let mut lfn = HashMap::new();

    let v1 = Var::Var("v1".to_string());
    let l1 = heap.add((make_num(1), 1));
    let v2 = Var::Var("v2".to_string());
    let l2 = heap.add((make_num(2), 1));

    let cases = vec![FnBody::Ret(v1.clone()), FnBody::Ret(v2.clone())];
    ctxt = ctxt.add(v1.clone(), l1.clone());
    ctxt = ctxt.add(v2.clone(), l2.clone());

    let var =  Var::Var("var".to_string());
    let l_var = heap.add((make_true(), 1));
    ctxt = ctxt.add(var.clone(), l_var.clone());

    let res = eval_case(var.clone(), cases, &ctxt, &mut heap, &mut lfn);
    assert_eq!(l2, res);
}


#[test]
fn test_const_app_full() {
    let mut heap = empty_heap();
    let mut ctxt = empty_ctxt();
    let mut lfn = HashMap::new();

    let v1 = Var::Var("v1".to_string());
    let l1 = heap.add((make_num(3), 1));
    let v2 = Var::Var("v2".to_string());
    let l2 = heap.add((make_num(2), 1));

    let expected = 1;

    ctxt = ctxt.add(v1.clone(), l1.clone());
    ctxt = ctxt.add(v2.clone(), l2.clone());

    let c = Const::Const("mod".to_string());
    let vars = vec![v1.clone(), v2.clone()];

    let res = eval_fncall(c, vars, &ctxt, &mut heap, &mut lfn);
    let n = get_num(res, &heap);
    assert_eq!(expected, n);
}

#[test]
fn test_const_app_part() {
    let mut heap = empty_heap();
    let mut ctxt = empty_ctxt();
    let mut lfn = HashMap::new();

    let v1 = Var::Var("v1".to_string());
    let l1 = heap.add((make_num(1), 1));

    ctxt = ctxt.add(v1.clone(), l1.clone());

    let c = Const::Const("div".to_string());
    let vars = vec![v1.clone()];

    let res = eval_fncall(c, vars, &ctxt, &mut heap, &mut lfn);
    let (c, ls) = get_pap(res, &heap);
    let Const::Const(nom) = c;
    assert_eq!("div".to_string(), nom);
    assert_eq!(1, ls.len());
    let n = get_num(ls[0], &heap);
    assert_eq!(1, n);
}

#[test]
fn test_proj2() {
    let mut heap = empty_heap();
    let mut ctxt = empty_ctxt();
    let mut lfn = HashMap::new();

    let var = Var::Var("var".to_string());
    let l1 = heap.add((make_num(1), 1));
    let l2 = heap.add((make_num(2), 1));

    let args = vec![l1.clone(), l2.clone()];
    let ctor = Value::Ctor(CONST_LIST, args);
    
    let l = heap.add((ctor, 1));
    ctxt = ctxt.add(var.clone(), l.clone());

    let res = eval_proj(2, var, &ctxt, &mut heap, &mut lfn);
    assert_eq!(l2, res);
}

#[test]
fn test_fibo_of_7(){
    let file_path = "./examples/fibo.pstl";
    let file_contents = fs::read_to_string(file_path)
        .expect(format!("unable to read file + {}", file_path).as_str());
    let parsed = reader::program().parse(file_contents).expect("can't parse");
    
    // fibo 7  = 21
    // fibo 10 = 89

    let n = 7;
    let expected = 21;
    
    let mut heap = empty_heap();
    let ctxt = add_value(Var::Var("n".to_string()), make_num(n), 
        add_value(Var::Var("m1".to_string()), make_num(1), empty_ctxt(), &mut heap), &mut heap);

    let call = Expr::FnCall(Const::Const("fibo".to_string()), vec![Var::Var("n".to_string()), Var::Var("m1".to_string())]);
    let res = start_interpreter(parsed, call, &ctxt, &mut heap);
    println!("fibo {} =", n);
    assert_eq!(expected, get_num(res, &heap));
}

#[test]
fn test_fibo_of_10(){
    let file_path = "./examples/fibo.pstl";
    let file_contents = fs::read_to_string(file_path)
        .expect(format!("unable to read file + {}", file_path).as_str());
    let parsed = reader::program().parse(file_contents).expect("can't parse");
    
    // fibo 7  = 21
    // fibo 10 = 89

    let n = 10;
    let expected = 89;
    
    let mut heap = empty_heap();
    let ctxt = add_value(Var::Var("n".to_string()), make_num(n), 
        add_value(Var::Var("m1".to_string()), make_num(1), empty_ctxt(), &mut heap), &mut heap);

    let call = Expr::FnCall(Const::Const("fibo".to_string()), vec![Var::Var("n".to_string()), Var::Var("m1".to_string())]);
    let res = start_interpreter(parsed, call, &ctxt, &mut heap);
    println!("fibo {} =", n);
    assert_eq!(expected, get_num(res, &heap));
}

#[test]
fn test_fibo_num_of_10(){
    let file_path = "./examples/fibo_num.pstl";
    let file_contents = fs::read_to_string(file_path)
        .expect(format!("unable to read file + {}", file_path).as_str());
    let parsed = reader::program().parse(file_contents).expect("can't parse");
    
    // fibo 7  = 21
    // fibo 10 = 89

    let n = 10;
    let expected = 89;
    
    let mut heap = empty_heap();
    let ctxt = add_value(Var::Var("n".to_string()), make_num(n), empty_ctxt(), &mut heap);

    let call = Expr::FnCall(Const::Const("fibo".to_string()), vec![Var::Var("n".to_string())]);
    let res = start_interpreter(parsed, call, &ctxt, &mut heap);
    println!("fibo {} =", n);
    assert_eq!(expected, get_num(res, &heap));
}

#[test]
fn test_fibo_num_of_7(){
    let file_path = "./examples/fibo_num.pstl";
    let file_contents = fs::read_to_string(file_path)
        .expect(format!("unable to read file + {}", file_path).as_str());
    let parsed = reader::program().parse(file_contents).expect("can't parse");
    
    // fibo 7  = 21
    // fibo 10 = 89

    let n = 7;
    let expected = 21;
    
    let mut heap = empty_heap();
    let ctxt = add_value(Var::Var("n".to_string()), make_num(n), empty_ctxt(), &mut heap);

    let call = Expr::FnCall(Const::Const("fibo".to_string()), vec![Var::Var("n".to_string())]);
    let res = start_interpreter(parsed, call, &ctxt, &mut heap);
    println!("fibo {} =", n);
    assert_eq!(expected, get_num(res, &heap));
}

#[test]
fn test_pap(){
    let file_path = "./examples/pap.pstl";
    let file_contents = fs::read_to_string(file_path)
        .expect(format!("unable to read file + {}", file_path).as_str());
    let parsed = reader::program().parse(file_contents).expect("can't parse");
    let mut heap = empty_heap();
    let ctxt = add_value(Var::Var("n".to_string()), make_num(10), 
                add_value(Var::Var("m".to_string()), make_num(6), empty_ctxt(), &mut heap)
            , &mut heap);

    let call = Expr::FnCall(Const::Const("papcall".to_string()), vec![Var::Var("n".to_string()), Var::Var("m".to_string())]);
    let res = start_interpreter(parsed, call, &ctxt, &mut heap);
    //println!("pap 10 6 = 4");
    let expected = 4;
    //dbg!(res);
    assert_eq!(expected, get_num(res, &heap));
}

#[test]
fn test_swap_pstl(){
    let file_path = "./examples/swap_pstl.pstl";
    let file_contents = fs::read_to_string(file_path)
        .expect(format!("unable to read file + {}", file_path).as_str());
    let parsed = reader::program().parse(file_contents).expect("can't parse");
    let mut heap = empty_heap();

    let l22 = heap.add((make_num(3), 1));
    let l21 = heap.add((make_num(2), 1));
    let l2 = heap.add((make_list(vec![l21, l22]), 1));
    let l11 = heap.add((make_num(1), 1));
    let l1 = heap.add((make_list(vec![l11, l2]), 1));

    let ctxt = empty_ctxt().add(Var::Var("l".to_string()), l1);

    dbg!(parsed.clone());

    let call = Expr::FnCall(Const::Const("swap".to_string()), vec![Var::Var("l".to_string())]);
    let res = start_interpreter(parsed, call, &ctxt, &mut heap);
    let (_, list) = get_ctor(res, &heap);
    
    let n2 = get_num(list[0], &heap);
    
    let (_, next_list) = get_ctor(list[1], &heap);

    let n1 = get_num(next_list[0], &heap);
    let n3 = get_num(next_list[1], &heap);

    dbg!(n1);
    dbg!(n2);
    dbg!(n3);
    
    //assert_eq!(expected, get_num(res, &heap));
}

#[test]
fn test_swap_pstl_2fun(){
    let file_path = "./examples/swap_pstl_2fun.pstl";
    let file_contents = fs::read_to_string(file_path)
        .expect(format!("unable to read file + {}", file_path).as_str());
    let parsed = reader::program().parse(file_contents).expect("can't parse");
    let mut heap = empty_heap();

    let l22 = heap.add((make_num(3), 1));
    let l21 = heap.add((make_num(2), 1));
    let l2 = heap.add((make_list(vec![l21, l22]), 1));
    let l11 = heap.add((make_num(1), 1));
    let l1 = heap.add((make_list(vec![l11, l2]), 1));

    let ctxt = empty_ctxt().add(Var::Var("l".to_string()), l1);

    dbg!(parsed.clone());

    let call = Expr::FnCall(Const::Const("swap".to_string()), vec![Var::Var("l".to_string())]);
    let res = start_interpreter(parsed, call, &ctxt, &mut heap);
    let (_, list) = get_ctor(res, &heap);
    
    let n2 = get_num(list[0], &heap);
    
    let (_, next_list) = get_ctor(list[1], &heap);

    let n1 = get_num(next_list[0], &heap);
    let n3 = get_num(next_list[1], &heap);
    
    assert_eq!(1, n1);
    assert_eq!(2, n2);
    assert_eq!(3, n3);
}

/*
#[test]
fn test_inc() {
    let mut heap = empty_heap();
    let mut ctxt = empty_ctxt();
    let mut lfn = HashMap::new();

    let var = Var::Var("var".to_string());
    let l = heap.add((make_num(1), 1));
    let fnbody = FnBody::Ret(var.clone());

    ctxt = ctxt.add(var.clone(), l.clone());

    let res = eval_inc(var, fnbody, &ctxt, &mut heap, &mut lfn);
    let (v, n) = heap.get(res.clone());
    assert_eq!(l, res);
    assert_eq!(2, n);
    assert_eq!(make_num(1), v);
}

#[test]
fn test_dec() {
    let mut heap = empty_heap();
    let mut ctxt = empty_ctxt();
    let mut lfn = HashMap::new();

    let var = Var::Var("var".to_string());
    let l = heap.add((make_num(1), 2));
    let fnbody = FnBody::Ret(var.clone());

    ctxt = ctxt.add(var.clone(), l.clone());

    let res = eval_dec(var, fnbody, &ctxt, &mut heap, &mut lfn);
    let (v, n) = heap.get(res.clone());
    assert_eq!(l, res);
    assert_eq!(1, n);
    assert_eq!(make_num(1), v);
}
*/