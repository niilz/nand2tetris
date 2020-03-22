use std::collections::HashMap;
use crate::tokenizer::token::{ Token, TokenType };

pub fn lookup(var: &Token, class_table: &ClassTable, subroutine_table: &SubroutineTable) -> Var {
    match var.token_type {
        TokenType::IntegerConstant => {
            let value = var.value.parse::<u32>().unwrap();
            Var::new("constant", "_", value)
        },
        TokenType::Identifier => {
            match subroutine_table.get(&var.value) {
                Some(var) => var,
                None => match class_table.get(&var.value) {
                    Some(var) => var,
                    None => panic!("Variable '{:?}' has not been declared.", var.value),
                }
            }
        },
        _ => panic!("Lookup for token-type '{:?}' with value '{}' is not implemented", var.token_type, var.value),
    }
}

pub fn get_object_type(identifier: &str, class_table: &ClassTable, subroutine_table: &SubroutineTable) -> String {
    match subroutine_table.get(identifier) {
        Some(var) => var.typ,
        None => match class_table.get(identifier) {
            Some(var) => var.typ,
            None => panic!("Identifier '{}' has not been declared", identifier),
        }
    }
}

pub fn is_object(identifier: &str, class_table: &ClassTable, subroutine_table: &SubroutineTable) -> bool {
    match subroutine_table.get(identifier) {
        Some(_) => true,
        None => match class_table.get(identifier) {
            Some(_) => true,
            None => false,
        }
    }
}

#[derive(Default)]
pub struct ClassTable {
    fields: HashMap<String, Var>,
    statiks: HashMap<String,Var>,
}

impl ClassTable {
    pub fn add(&mut self, name: &str, var: Var) {
        match var.kind.as_ref() {
            "field" => {
                self.fields.insert(name.to_string(), var);
            },
            "static" => {
                self.statiks.insert(name.to_string(), var);
            },
            _ => panic!("ClassVarDecs must be of kind field or static."),
        }
    }
    pub fn get(&self, var_name: &str) -> Option<Var> {
        let field_var = self.fields.get(var_name);
        let statik_var = self.statiks.get(var_name);
        match (field_var, statik_var) {
            (Some(var), _) => Some(var.clone()),
            (_, Some(var)) => Some(var.clone()),
            _ => None,
        }
    }
    pub fn get_next_idx(&self, var_kind: &str) -> u32 {
        match var_kind {
            "field" => self.fields.len() as u32,
            "static" => self.statiks.len() as u32,
            _ => panic!("invalid class-var-kind of '{}' has been passed.", var_kind),
        }
    }
    pub fn get_field_count(&self) -> usize {
        self.fields.len()
    }
}

#[derive(Default)]
pub struct SubroutineTable {
    pub args: HashMap<String, Var>,
    pub locals: HashMap<String, Var>,
}

impl SubroutineTable {
    pub fn add(&mut self, name: &str, var: Var) {
        match var.kind.as_ref() {
            "argument" => {
                self.args.insert(name.to_string(), var);
            },
            "local" => {
                self.locals.insert(name.to_string(), var);
            },
            _ => panic!("Subroutine vars must be of kind arg or local."),
        }
    }
    pub fn get(&self, var_name: &str) -> Option<Var> {
        let arg_var = self.args.get(var_name);
        let local_var = self.locals.get(var_name);
        match (arg_var, local_var) {
            (Some(var), _) => Some(var.clone()),
            (_, Some(var)) => Some(var.clone()),
            _ => None,
        }
    }
    pub fn get_next_idx(&self, var_kind: &str) -> u32 {
        match var_kind {
            "argument" => self.args.len() as u32,
            "local" => self.locals.len() as u32,
            _ => panic!("invalid subroutine-var-kind of '{}' has been passed.", var_kind),
        }
    }
    pub fn get_local_var_count(&self) -> usize {
        self.get_next_idx("local") as usize
    } 
    pub fn has_this(&self) -> bool {
        !self.args.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Var {
    pub kind: String,
    pub typ: String,
    pub idx: u32,
}
impl Var {
    pub fn new(kind: &str, typ: &str, idx: u32) -> Self {
        Var {
            idx,
            kind: kind.to_string(),
            typ: typ.to_string(),
        }
    }
    pub fn to_xml(&self) -> String {
        // format!(
        //     "<VAR_DATA>k: {:?} t: {:?} i: {:?}</VAR_DATA>",
        //     self.kind, self.typ, self.idx.to_string())
        "<VAR_DATA> k: ".to_string() + &self.kind
        +  " t: " + &self.typ
        + " i: " + &self.idx.to_string()
        + " </VAR_DATA>"
    }
}

// TESTS
#[test]
fn class_var_gets_created() {
    let dummy_var_left = Var {
        kind: "static".to_string(),
        typ: "int".to_string(),
        idx: 0,
    };
    let dummy_var_right = Var::new("static", "int", 0);
    assert_eq!(dummy_var_left, dummy_var_right);
}

#[test]
fn class_vars_get_stored() {
    let dummy_var = Var::new("static", "int", 3);
    let mut class_table = ClassTable::default();
    class_table.add("num", dummy_var);
    assert_eq!(class_table.get("num").unwrap().idx, 3);
}

#[test]
fn subroutine_vars_get_stored() {
    let dummy_var = Var::new("arg", "int", 1);
    let mut subroutine_table = SubroutineTable::default();
    subroutine_table.add("pups", dummy_var);
    assert_eq!(subroutine_table.get("pups").unwrap().idx, 1);
}

#[test]
fn empty_args_is_empty() {
    let subroutine_table = SubroutineTable::default();
    assert_eq!(subroutine_table.has_this(), false);
}

#[test]
fn args_with_this_is_not_empty() {
    let dummy_var = Var::new("arg", "class_name", 0);
    let mut subroutine_table = SubroutineTable::default();
    subroutine_table.add("this", dummy_var);
    assert_eq!(subroutine_table.has_this(), true);
}

#[test]
fn get_next_idx_on_class_table_works() {
    let mut class_table = ClassTable::default();
    assert_eq!(class_table.get_next_idx("field"), 0);
    let dummy_var = Var::new("field", "class_name", 0);
    class_table.add("myType", dummy_var.clone());
    assert_eq!(class_table.get_next_idx("field"), 1);
    assert_eq!(class_table.get_next_idx("static"), 0);
}