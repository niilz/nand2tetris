use std::collections::HashMap;

#[derive(Default)]
pub struct ClassTable {
    fields: HashMap<String, Var>,
    statiks: HashMap<String,Var>,
}

impl ClassTable {
    pub fn add(&mut self, var: Var) {
        match var.kind.as_ref() {
            "field" => {
                self.fields.insert(var.name.to_string(), var);
            },
            "static" => {
                self.statiks.insert(var.name.to_string(), var);
            },
            _ => panic!("ClassVarDecs must be of kind field or static."),
        }
    }
    pub fn get(&self, var_name: &str) -> Var {
        let field_var = self.fields.get(var_name);
        let statik_var = self.statiks.get(var_name);
        match (field_var, statik_var) {
            (Some(var), _) => var.clone(),
            (_, Some(var)) => var.clone(),
            _ => panic!("No variable with name '{}' in class", var_name),
        }
    }
}

#[derive(Default)]
pub struct SubroutineTable {
    args: HashMap<String, Var>,
    locals: HashMap<String, Var>,
}

impl SubroutineTable {
    pub fn add(&mut self, var: Var) {
        match var.kind.as_ref() {
            "arg" => {
                self.args.insert(var.name.to_string(), var);
            },
            "local" => {
                self.locals.insert(var.name.to_string(), var);
            },
            _ => panic!("ClassVarDecs must be of kind field or static."),
        }
    }
    pub fn get(&self, var_name: &str) -> Var {
        let arg_var = self.args.get(var_name);
        let local_var = self.locals.get(var_name);
        match (arg_var, local_var) {
            (Some(var), _) => var.clone(),
            (_, Some(var)) => var.clone(),
            _ => panic!("No variable with name '{}' in class", var_name),
        }
    }
    pub fn has_this(&self) -> bool {
        !self.args.is_empty()
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
    pub fn new(kind: &str, typ: &str, name: &str, idx: u32) -> Self {
        Var {
            idx,
            kind: kind.to_string(),
            typ: typ.to_string(),
            name: name.to_string(),
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
    let dummy_var_right = Var::new("static", "int", "num", 0);
    assert_eq!(dummy_var_left, dummy_var_right);
}

#[test]
fn class_vars_get_stored() {
    let dummy_var = Var::new("static", "int", "num", 3);
    let mut class_table = ClassTable::default();
    class_table.add(dummy_var);
    assert_eq!(class_table.get("num").idx, 3);
}

#[test]
fn subroutine_vars_get_stored() {
    let dummy_var = Var::new("arg", "int", "pups", 1);
    let mut subroutine_table = SubroutineTable::default();
    subroutine_table.add(dummy_var);
    assert_eq!(subroutine_table.get("pups").idx, 1);
}

#[test]
fn empty_args_is_empty() {
    let subroutine_table = SubroutineTable::default();
    assert_eq!(subroutine_table.has_this(), false);
}
#[test]
fn args_with_this_is_not_empty() {
    let dummy_var = Var::new("arg", "class_name", "this", 0);
    let mut subroutine_table = SubroutineTable::default();
    subroutine_table.add(dummy_var);
    assert_eq!(subroutine_table.has_this(), true);
}