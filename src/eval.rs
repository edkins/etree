use anyhow::bail;
use bigdecimal::BigDecimal;
use log::trace;
use std::collections::HashMap;
use crate::ast::{Block,Expr,Program,Statement};

#[derive(Clone, Eq, PartialEq)]
enum Tree {
    FreeVar(String),
    Num(BigDecimal),
    Call(String, Vec<Tree>),
}

impl std::fmt::Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Tree::FreeVar(x) => write!(f, "'{}", x),
            Tree::Num(d) => write!(f, "{}", d),
            Tree::Call(fun, ts) => {
                if !fun.chars().next().unwrap().is_alphabetic() && ts.len() == 2 {
                    write!(f, "({:?} {} {:?})", ts[0], fun, ts[1])
                } else {
                    write!(f, "{}{:?}", fun, ts)
                }
            }
        }
    }
}

impl Program {
    fn eval(&self, env: &HashMap<String,Tree>, e: &Expr) -> anyhow::Result<Tree> {
        match e {
            Expr::Var(x) => {
                if let Some(t) = env.get(x) {
                    Ok(t.clone())
                } else {
                    bail!("Undefined variable {}", x);
                }
            }
            Expr::FreeVar(x) => Ok(Tree::FreeVar(x.clone())),
            Expr::Num(d) => Ok(Tree::Num(d.clone())),
            Expr::Call(f, es) => {
                let ts = es.iter().map(|e2|self.eval(env, e2)).collect::<Result<_,_>>()?;
                Ok(Tree::Call(f.clone(), ts))
            }
        }
    }

    fn perform(&self, stmts: &[Statement]) -> anyhow::Result<()> {
        let mut env = HashMap::new();
        for stmt in stmts {
            let Statement::Assign(x, e) = stmt;
            if env.contains_key(x) {
                bail!("Already defined variable {}", x);
            }
            let t = self.eval(&env, e)?;
            trace!("{} = {:?}", x, t);
            env.insert(x.clone(), t);
        }
        Ok(())
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let mut found = false;
        for block in &self.0 {
            let Block::Fun(name, args, stmts) = block;
            if name == "main" && args.is_empty() {
                if found {
                    bail!("Multiple main functions");
                }
                self.perform(&stmts)?;
                found = true;
            }
        }
        if !found {
            bail!("Could not find main function");
        }
        Ok(())
    }
}
