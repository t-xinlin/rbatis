use serde_json::{json, Value};

use crate::ast::ast::RbatisAST;
use crate::ast::node::node::{create_deep, do_child_nodes, print_child, SqlNodePrint};
use crate::ast::node::node_type::NodeType;
use crate::convert::stmt_convert::StmtConvert;
use crate::engine::runtime::RbatisEngine;

#[derive(Clone, Debug)]
pub struct WhereNode {
    pub childs: Vec<NodeType>,
}

impl RbatisAST for WhereNode {
    fn eval(&self, convert: &impl StmtConvert, env: &mut Value, engine: &RbatisEngine, arg_array: &mut Vec<Value>) -> Result<String, rbatis_core::Error> {
        let result = do_child_nodes(convert, &self.childs, env, engine, arg_array)?;
        let s = result.trim();
        if s.is_empty() {
            return Result::Ok(" ".to_string());
        } else {
            return Result::Ok(" where ".to_string() + s.trim_start_matches("and "));
        }
    }
}

impl SqlNodePrint for WhereNode {
    fn print(&self, deep: i32) -> String {
        let mut result = create_deep(deep) + "<where";
        result = result + ">";
        result = result + print_child(self.childs.as_ref(), deep + 1).as_str();
        result = result + create_deep(deep).as_str() + "</where>";
        return result;
    }
}