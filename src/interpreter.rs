use crate::ast::*;
use crate::bawk_type::BawkType;

use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};

pub fn interp<'a>(p: &'a Program, file_path: &String) -> io::Result<()> {
    let mut env: HashMap<String, BawkType> = HashMap::new();

    let file = std::fs::File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().peekable();
    let mut line_number = 0;
    let mut highest_key = 0;

    while let Some(line_res) = lines.next() {
        let line = line_res.unwrap();
        line_number += 1; 
        env.insert("NR".to_string(), BawkType::Integer(line_number as i64));

        let line_parts = line.split_whitespace();

        let mut current_key = 0;
        for line_part in line_parts {
            current_key += 1;
            let key = format!("${}", current_key);
            env.insert(key, BawkType::Text(String::from(line_part)));

            if current_key > highest_key { 
                highest_key = current_key;
            }
        }

        if current_key < highest_key {
            for i in current_key+1..highest_key+1 {
                env.remove(&format!("${}", i));
            }
        }

        eprintln!("Handling line {}", line_number);

        for rule in &p.rules {
            match rule.rule_type {
                RuleType::BeginRule => {
                    if line_number != 0 { continue; }
                },
                RuleType::EndRule => {
                    if !lines.peek().is_none() { continue; } 
                },
                _ => { () /* Just continue as normal, implement pattern rule here later */ }
            }

            for stmt in &rule.statements {
                for expr in &stmt.expressions {
                    interp_expr(&mut env, &expr, &line);
                }
            }
        }

        eprintln!("env {:#?}", env);
    }

    Ok(())
}

fn interp_expr<'a>(env: &mut HashMap<String, BawkType>, expr: &'a Expr, l: &str) -> BawkType {
    use crate::ast::Expr_::*;

    match expr.node {
        Assign(ref var, ref b) => {
            let val = interp_expr(env, b, l);
            env.insert(var.clone(), val.clone());
            val
        }
        Var(ref var) => env.get(var).unwrap().clone(),
        Text(ref lit) => { let val = lit.clone(); BawkType::Text(val) },
        Integer(lit) => BawkType::Integer(lit),
        Print(ref e) => {
            let val = interp_expr(env, e, l);
            println!("{}", val);
            val
        },
        _ => { panic!("UNIMPLEMENTED!") }
    }
}
