use std::fmt::Write;
use crate::node::{Node, Data, ParseType, ParseTypeBase};
use crate::token::Token;

pub struct ASTContext<'a> {
    pub tokens: &'a [Token],
    pub source: &'a str,
}

pub struct ASTPrinter<'a> {
    ctx: &'a ASTContext<'a>,
}

impl<'a> ASTPrinter<'a> {
    pub fn new(ctx: &'a ASTContext<'a>) -> Self {
        Self { ctx }
    }

    pub fn print(&self, nodes: &[Node]) -> String {
        let mut output = String::new();
        for node in nodes {
            self.print_node(node, 0, &mut output);
        }
        output
    }

    fn resolve_token(&self, idx: usize) -> &str {
        if let Some(token) = self.ctx.tokens.get(idx) {
            self.ctx.source.get(token.span.clone()).unwrap_or("<invalid_span>")
        } else {
            "<invalid_index>"
        }
    }

    fn print_node(&self, node: &Node, indent: usize, out: &mut String) {
        let indentation = "  ".repeat(indent);
        let _ = write!(out, "{}[{:?}] ", indentation, node.span);

        match &node.data {
            Data::Identifier(idx) => {
                let _ = writeln!(out, "Identifier: {}", self.resolve_token(*idx));
            }
            Data::String(idx) => {
                let _ = writeln!(out, "String: \"{}\"", self.resolve_token(*idx));
            }
            Data::Integer(idx) => {
                let _ = writeln!(out, "Integer: {}", self.resolve_token(*idx));
            }
            Data::Float(idx) => {
                let _ = writeln!(out, "Float: {}", self.resolve_token(*idx));
            }
            Data::BinaryOp { lhs, op, rhs } => {
                let _ = writeln!(out, "BinaryOp ({:?})", op);
                self.print_node(lhs, indent + 1, out);
                self.print_node(rhs, indent + 1, out);
            }
            Data::UnaryOp { op, target } => {
                let _ = writeln!(out, "UnaryOp ({:?})", op);
                self.print_node(target, indent + 1, out);
            }
            Data::VarDeclaration { name, mutable, ty, value } => {
                let mut_str = if *mutable { "mut " } else { "" };
                let name_str = self.resolve_token(*name);
                let _ = writeln!(out, "VarDecl: {}{} : {}", mut_str, name_str, self.format_type(ty));
                self.print_node(value, indent + 1, out);
            }
            Data::FnDeclaration { name, params, return_type, body } => {
                let name_str = self.resolve_token(*name);
                let _ = writeln!(out, "FnDecl: {} -> {}", name_str, self.format_type(return_type));

                if let Some(p_list) = params {
                    for param in p_list {
                        self.print_node(param, indent + 1, out);
                    }
                }
                for stmt in body {
                    self.print_node(stmt, indent + 1, out);
                }
            }
            Data::FnParameter { ty, name } => {
                let _ = writeln!(out, "Param: {} : {}", self.resolve_token(*name), self.format_type(ty));
            }
        }
    }

    fn format_type(&self, parse_type: &ParseType) -> String {
        let mut modifiers = String::new();
        if parse_type.unsigned { modifiers.push_str("unsigned "); }
        if parse_type.long { modifiers.push_str("long "); }

        let base_str = match &parse_type.base {
            ParseTypeBase::Char => "char".to_string(),
            ParseTypeBase::Void => "void".to_string(),
            ParseTypeBase::Byte => "byte".to_string(),
            ParseTypeBase::Short => "short".to_string(),
            ParseTypeBase::Int => "int".to_string(),
            ParseTypeBase::Float => "float".to_string(),
            ParseTypeBase::Pointer(inner) => format!("*{}", self.format_type(inner)),
            ParseTypeBase::Array(inner, size_node) => {
                let mut size_str = String::new();
                self.print_node(size_node, 0, &mut size_str);
                format!("[{}; {}]", self.format_type(inner), size_str.trim())
            }
            ParseTypeBase::Function { params, return_type, variadic } => {
                let param_types: Vec<String> = params.iter().map(|p| self.format_type(p)).collect();
                let var_str = if *variadic { ", ..." } else { "" };
                format!("fn({}){} -> {}", param_types.join(", "), var_str, self.format_type(return_type))
            }
            ParseTypeBase::Struct(span) => format!("struct<span: {:?}>", span),
            ParseTypeBase::Union(span) => format!("union<span: {:?}>", span),
            ParseTypeBase::Enum(span) => format!("enum<span: {:?}>", span),
            ParseTypeBase::Alias(span) => format!("alias<span: {:?}>", span),
        };

        format!("{}{}", modifiers, base_str)
    }
}
