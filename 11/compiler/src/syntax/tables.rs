use std::collections::HashMap;
use super::token::{ Token, TokenType };

#[derive(Default)]
pub struct ClassTable {
    field_count: u32,
    statik_count: u32,
    fields: VarMap,
    statiks: VarMap,
}

impl ClassTable {
    pub fn add(&mut self, var: &mut Var) {
        match var.kind.as_ref() {
            "field" => self.fields.update(var),
            "static" => self.statiks.update(var),
            _ => panic!("ClassVarDecs must be of kind field or static."),
        }
    }
    pub fn get(&self, var_name: &str) -> Var {
        println!("{:?}", var_name);
        let field_var = self.fields.vars.get(var_name);
        let statik_var = self.statiks.vars.get(var_name);
        println!("{:?}", field_var);
        println!("{:?}", statik_var);
        match (field_var, statik_var) {
            (Some(var), _) => var.clone(),
            (_, Some(var)) => var.clone(),
            _ => panic!("No variable with name '{}' in class", var_name),
        }
    }
}

#[derive(Default)]
struct VarMap {
    count: u32,
    vars: HashMap<String, Var>,
}
impl VarMap {
    fn update(&mut self, var: &mut Var) {
        if self.vars.contains_key(&var.name) {
            panic!("Var has already been declared!");
        }
        self.count += 1;
        var.idx = self.count;
        self.vars.insert(var.name.to_string(), var.clone());
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Var {
    kind: String,
    typ: String,
    name: String,
    idx: u32,
}
impl Var {
    pub fn new(kind: String, typ: String, name: String) -> Self {
        Var {
            idx: 0,
            kind,
            typ,
            name,
        }
    }
    pub fn to_xml(&self) -> String {
        format!(
            "<VAR_DATA>k: {} t: {} n: {} i: {}</VAR_DATA>",
            self.kind, self.typ, self.name, self.idx)
    }
}

// TESTS
#[test]
fn class_var_gets_created() {
    let dummy_var_left = Var {
        kind: "static".to_string(),
        typ: "int".to_string(),
        name: "num".to_string(),
        idx: 0,
    };
    let dummy_var_right = Var::new("static".to_string(), "int".to_string(), "num".to_string());
    assert_eq!(dummy_var_left, dummy_var_right);
}

#[test]
fn class_vars_update() {
    let mut var_map = VarMap {
        count: 0,
        vars: HashMap::new(),
    };
    let mut dummy_var = Var::new("static".to_string(), "int".to_string(), "num".to_string());
    var_map.update(&mut dummy_var);
    assert_eq!(var_map.count, 1);
    assert_eq!(var_map.vars.get("num").unwrap().idx, 1);
}