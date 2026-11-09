//! Abstract Syntax Tree (AST) definitions for the FSX language.

use crate::token::Span;

#[derive(Clone, Debug, PartialEq)]
pub struct SourceFile {
    pub imports: Vec<ImportDecl>,
    pub types: Vec<TypeDecl>,
    pub components: Vec<ComponentDecl>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImportDecl {
    pub items: Vec<String>,
    pub source: String,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeDecl {
    pub name: String,
    pub fields: Vec<TypeField>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeField {
    pub name: String,
    pub type_ref: TypeRef,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeRef {
    Primitive(String),
    Custom(String),
    List(Box<TypeRef>),
    Option(Box<TypeRef>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ComponentDecl {
    pub name: String,
    pub props_param: Option<(String, TypeRef)>,
    pub statements: Vec<Statement>,
    pub return_elem: FsxElement,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Hook(HookStmt),
    Let {
        ident: String,
        type_ref: Option<TypeRef>,
        value: Expr,
        span: Span,
    },
    Expr(Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub struct HookStmt {
    pub ident: String,
    pub hook_name: String,
    pub args: Vec<Expr>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FsxElement {
    pub tag_name: String,
    pub attributes: Vec<FsxAttribute>,
    pub children: Vec<FsxChild>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FsxAttribute {
    pub name: String,
    pub value: AttrValue,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AttrValue {
    Literal(Expr),
    Expr(Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub enum FsxChild {
    Element(FsxElement),
    Text(String),
    Expr(Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Ident(String, Span),
    Int(i64, Span),
    Float(f64, Span),
    StringLit(String, Span),
    Bool(bool, Span),
    Member(Box<Expr>, String, Span),
    Call(Box<Expr>, Vec<Expr>, Span),
    ArrowFn {
        params: Vec<String>,
        body: Box<Expr>,
        span: Span,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
        span: Span,
    },
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Self::Ident(_, s)
            | Self::Int(_, s)
            | Self::Float(_, s)
            | Self::StringLit(_, s)
            | Self::Bool(_, s)
            | Self::Member(_, _, s)
            | Self::Call(_, _, s)
            | Self::ArrowFn { span: s, .. }
            | Self::Binary { span: s, .. } => *s,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
    Lt,
    Gt,
}
