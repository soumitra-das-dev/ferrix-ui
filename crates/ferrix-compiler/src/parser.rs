//! Recursive-descent parser for the FSX declarative language.

use crate::ast::*;
use crate::lexer::{LexError, Lexer};
use crate::token::{Span, Token, TokenKind};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("Lexer error: {0}")]
    Lex(#[from] LexError),
    #[error("Expected token '{expected}', found '{found:?}' at line {line}, col {col}")]
    UnexpectedToken {
        expected: &'static str,
        found: TokenKind,
        line: usize,
        col: usize,
    },
    #[error("Mismatched closing tag: expected '</{expected}>', found '</{found}>' at line {line}, col {col}")]
    MismatchedClosingTag {
        expected: String,
        found: String,
        line: usize,
        col: usize,
    },
    #[error("Unexpected end of file while parsing {0}")]
    UnexpectedEof(&'static str),
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
    peek_token: Option<Token>,
    tag_depth: usize,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Lexer::new(source);
        let current = lexer.next_token()?;
        Ok(Self {
            lexer,
            current,
            peek_token: None,
            tag_depth: 0,
        })
    }

    fn current_span(&self) -> Span {
        self.current.span
    }

    fn advance(&mut self) -> Result<Token, ParseError> {
        let prev = std::mem::replace(
            &mut self.current,
            if let Some(t) = self.peek_token.take() {
                t
            } else {
                self.lexer.next_token()?
            },
        );
        Ok(prev)
    }

    fn expect(
        &mut self,
        expected_kind: &TokenKind,
        expected_name: &'static str,
    ) -> Result<Token, ParseError> {
        if &self.current.kind == expected_kind {
            self.advance()
        } else {
            Err(ParseError::UnexpectedToken {
                expected: expected_name,
                found: self.current.kind.clone(),
                line: self.current.span.line,
                col: self.current.span.col,
            })
        }
    }

    fn expect_ident(&mut self) -> Result<(String, Span), ParseError> {
        match self.current.kind.clone() {
            TokenKind::Ident(s) => {
                let span = self.current.span;
                self.advance()?;
                Ok((s, span))
            }
            _ => Err(ParseError::UnexpectedToken {
                expected: "identifier",
                found: self.current.kind.clone(),
                line: self.current.span.line,
                col: self.current.span.col,
            }),
        }
    }

    /// Parses the entire source file.
    pub fn parse_source_file(&mut self) -> Result<SourceFile, ParseError> {
        let mut imports = Vec::new();
        let mut types = Vec::new();
        let mut components = Vec::new();

        while self.current.kind != TokenKind::Eof {
            match self.current.kind {
                TokenKind::Import => {
                    imports.push(self.parse_import_decl()?);
                }
                TokenKind::Type => {
                    types.push(self.parse_type_decl()?);
                }
                TokenKind::Export | TokenKind::Function => {
                    components.push(self.parse_component_decl()?);
                }
                _ => {
                    return Err(ParseError::UnexpectedToken {
                        expected: "top-level declaration (import, type, or function)",
                        found: self.current.kind.clone(),
                        line: self.current.span.line,
                        col: self.current.span.col,
                    })
                }
            }
        }

        Ok(SourceFile {
            imports,
            types,
            components,
        })
    }

    // import { A, B } from "path";
    fn parse_import_decl(&mut self) -> Result<ImportDecl, ParseError> {
        let start_span = self.expect(&TokenKind::Import, "import")?.span;
        self.expect(&TokenKind::OpenBrace, "{")?;

        let mut items = Vec::new();
        while self.current.kind != TokenKind::CloseBrace && self.current.kind != TokenKind::Eof {
            let (item, _) = self.expect_ident()?;
            items.push(item);
            if self.current.kind == TokenKind::Comma {
                self.advance()?;
            } else {
                break;
            }
        }

        self.expect(&TokenKind::CloseBrace, "}")?;
        self.expect(&TokenKind::From, "from")?;

        let source = match self.current.kind.clone() {
            TokenKind::StringLit(s) => {
                self.advance()?;
                s
            }
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: "string literal path",
                    found: self.current.kind.clone(),
                    line: self.current.span.line,
                    col: self.current.span.col,
                })
            }
        };

        let end_span = self.expect(&TokenKind::Semi, ";")?.span;
        let span = Span::new(
            start_span.start,
            end_span.end,
            start_span.line,
            start_span.col,
        );

        Ok(ImportDecl {
            items,
            source,
            span,
        })
    }

    // type CounterProps = { initial: i32 };
    fn parse_type_decl(&mut self) -> Result<TypeDecl, ParseError> {
        let start_span = self.expect(&TokenKind::Type, "type")?.span;
        let (name, _) = self.expect_ident()?;
        self.expect(&TokenKind::Equal, "=")?;
        self.expect(&TokenKind::OpenBrace, "{")?;

        let mut fields = Vec::new();
        while self.current.kind != TokenKind::CloseBrace && self.current.kind != TokenKind::Eof {
            let (field_name, _) = self.expect_ident()?;
            self.expect(&TokenKind::Colon, ":")?;
            let (type_name, _) = self.expect_ident()?;
            fields.push(TypeField {
                name: field_name,
                type_ref: TypeRef::Primitive(type_name),
            });

            if self.current.kind == TokenKind::Comma || self.current.kind == TokenKind::Semi {
                self.advance()?;
            } else {
                break;
            }
        }

        self.expect(&TokenKind::CloseBrace, "}")?;
        let end_span = self.expect(&TokenKind::Semi, ";")?.span;
        let span = Span::new(
            start_span.start,
            end_span.end,
            start_span.line,
            start_span.col,
        );

        Ok(TypeDecl { name, fields, span })
    }

    // export? function Counter(props: CounterProps): Element { ... }
    fn parse_component_decl(&mut self) -> Result<ComponentDecl, ParseError> {
        let mut start_span = self.current_span();
        if self.current.kind == TokenKind::Export {
            start_span = self.advance()?.span;
        }

        self.expect(&TokenKind::Function, "function")?;
        let (name, _) = self.expect_ident()?;
        self.expect(&TokenKind::OpenParen, "(")?;

        let mut props_param = None;
        if self.current.kind != TokenKind::CloseParen {
            let (param_name, _) = self.expect_ident()?;
            self.expect(&TokenKind::Colon, ":")?;
            let (type_name, _) = self.expect_ident()?;
            props_param = Some((param_name, TypeRef::Custom(type_name)));
        }

        self.expect(&TokenKind::CloseParen, ")")?;
        self.expect(&TokenKind::Colon, ":")?;
        let (_ret_type, _) = self.expect_ident()?; // "Element"
        self.expect(&TokenKind::OpenBrace, "{")?;

        let mut statements = Vec::new();
        let mut return_elem = None;

        while self.current.kind != TokenKind::CloseBrace && self.current.kind != TokenKind::Eof {
            if self.current.kind == TokenKind::Return {
                self.advance()?; // consume "return"
                let elem = self.parse_return_element()?;
                return_elem = Some(elem);
                if self.current.kind == TokenKind::Semi {
                    self.advance()?;
                }
                break;
            } else if self.current.kind == TokenKind::Const || self.current.kind == TokenKind::Let {
                statements.push(self.parse_const_or_let_stmt()?);
            } else {
                let expr = self.parse_expr()?;
                statements.push(Statement::Expr(expr));
                if self.current.kind == TokenKind::Semi {
                    self.advance()?;
                }
            }
        }

        let end_span = self.expect(&TokenKind::CloseBrace, "}")?.span;
        let elem = return_elem.ok_or(ParseError::UnexpectedEof("return statement"))?;

        let span = Span::new(
            start_span.start,
            end_span.end,
            start_span.line,
            start_span.col,
        );

        Ok(ComponentDecl {
            name,
            props_param,
            statements,
            return_elem: elem,
            span,
        })
    }

    // const count = useState(props.initial);
    fn parse_const_or_let_stmt(&mut self) -> Result<Statement, ParseError> {
        let start_span = self.advance()?.span; // "const" or "let"
        let (ident, _) = self.expect_ident()?;
        self.expect(&TokenKind::Equal, "=")?;

        // Check if hook call (e.g. useState(...))
        if let TokenKind::Ident(hook_name) = self.current.kind.clone() {
            if is_hook_name(&hook_name) {
                let _hook_span = self.advance()?.span;
                self.expect(&TokenKind::OpenParen, "(")?;
                let mut args = Vec::new();
                while self.current.kind != TokenKind::CloseParen
                    && self.current.kind != TokenKind::Eof
                {
                    args.push(self.parse_expr()?);
                    if self.current.kind == TokenKind::Comma {
                        self.advance()?;
                    } else {
                        break;
                    }
                }
                self.expect(&TokenKind::CloseParen, ")")?;
                let end_span = self.expect(&TokenKind::Semi, ";")?.span;
                let span = Span::new(
                    start_span.start,
                    end_span.end,
                    start_span.line,
                    start_span.col,
                );
                return Ok(Statement::Hook(HookStmt {
                    ident,
                    hook_name,
                    args,
                    span,
                }));
            }
        }

        let expr = self.parse_expr()?;
        let end_span = self.expect(&TokenKind::Semi, ";")?.span;
        let span = Span::new(
            start_span.start,
            end_span.end,
            start_span.line,
            start_span.col,
        );

        Ok(Statement::Let {
            ident,
            type_ref: None,
            value: expr,
            span,
        })
    }

    // return (<Column ...> ... </Column>);
    fn parse_return_element(&mut self) -> Result<FsxElement, ParseError> {
        let has_paren = if self.current.kind == TokenKind::OpenParen {
            self.advance()?;
            true
        } else {
            false
        };

        let elem = self.parse_fsx_element()?;

        if has_paren {
            self.expect(&TokenKind::CloseParen, ")")?;
        }

        Ok(elem)
    }

    // <Tag attr={val}> ... </Tag> or <Tag attr={val} />
    pub fn parse_fsx_element(&mut self) -> Result<FsxElement, ParseError> {
        let start_span = if self.current.kind == TokenKind::Less {
            let span = self.current.span;
            self.current = self.lexer.next_token()?;
            span
        } else {
            return Err(ParseError::UnexpectedToken {
                expected: "<",
                found: self.current.kind.clone(),
                line: self.current.span.line,
                col: self.current.span.col,
            });
        };

        let (tag_name, _) = self.expect_ident()?;

        let mut attributes = Vec::new();
        while self.current.kind != TokenKind::Greater
            && self.current.kind != TokenKind::SlashGreater
            && self.current.kind != TokenKind::Eof
        {
            let attr_start = self.current_span();
            let (attr_name, _) = self.expect_ident()?;
            self.expect(&TokenKind::Equal, "=")?;

            let attr_value = if self.current.kind == TokenKind::OpenBrace {
                self.advance()?; // consume {
                let expr = self.parse_expr()?;
                self.expect(&TokenKind::CloseBrace, "}")?;
                AttrValue::Expr(expr)
            } else if let TokenKind::StringLit(s) = self.current.kind.clone() {
                let s_span = self.advance()?.span;
                AttrValue::Literal(Expr::StringLit(s, s_span))
            } else if let TokenKind::Int(i) = self.current.kind {
                let i_span = self.advance()?.span;
                AttrValue::Literal(Expr::Int(i, i_span))
            } else if let TokenKind::Float(f) = self.current.kind {
                let f_span = self.advance()?.span;
                AttrValue::Literal(Expr::Float(f, f_span))
            } else {
                return Err(ParseError::UnexpectedToken {
                    expected: "attribute value ({expr} or literal)",
                    found: self.current.kind.clone(),
                    line: self.current.span.line,
                    col: self.current.span.col,
                });
            };

            let attr_end = self.current_span();
            attributes.push(FsxAttribute {
                name: attr_name,
                value: attr_value,
                span: Span::new(
                    attr_start.start,
                    attr_end.end,
                    attr_start.line,
                    attr_start.col,
                ),
            });
        }

        if self.current.kind == TokenKind::SlashGreater {
            let end_span = self.current.span;
            if self.tag_depth > 0 {
                self.current = self.lexer.next_jsx_child()?;
            } else {
                self.current = self.lexer.next_token()?;
            }
            let span = Span::new(
                start_span.start,
                end_span.end,
                start_span.line,
                start_span.col,
            );
            return Ok(FsxElement {
                tag_name,
                attributes,
                children: Vec::new(),
                span,
            });
        }

        if self.current.kind != TokenKind::Greater {
            return Err(ParseError::UnexpectedToken {
                expected: ">",
                found: self.current.kind.clone(),
                line: self.current.span.line,
                col: self.current.span.col,
            });
        }

        // Entering element body
        self.tag_depth += 1;
        self.current = self.lexer.next_jsx_child()?;

        let mut children = Vec::new();
        loop {
            match self.current.kind.clone() {
                TokenKind::TextContent(text) => {
                    if let Some(cleaned) = clean_jsx_text(&text) {
                        children.push(FsxChild::Text(cleaned));
                    }
                    self.current = self.lexer.next_jsx_child()?;
                }
                TokenKind::OpenBrace => {
                    // Interpolated expression: {expr}
                    self.current = self.lexer.next_token()?;
                    let expr = self.parse_expr()?;
                    if self.current.kind == TokenKind::CloseBrace {
                        self.current = self.lexer.next_jsx_child()?;
                    } else {
                        return Err(ParseError::UnexpectedToken {
                            expected: "}",
                            found: self.current.kind.clone(),
                            line: self.current.span.line,
                            col: self.current.span.col,
                        });
                    }
                    children.push(FsxChild::Expr(expr));
                }
                TokenKind::Less => {
                    // Child element
                    let child_elem = self.parse_fsx_element()?;
                    children.push(FsxChild::Element(child_elem));
                }
                TokenKind::LessSlash => {
                    // Closing tag: </Tag>
                    self.tag_depth -= 1;
                    let closing_token = self.lexer.next_token()?;
                    let (closing_name, close_span) = match closing_token.kind {
                        TokenKind::Ident(s) => (s, closing_token.span),
                        other => {
                            return Err(ParseError::UnexpectedToken {
                                expected: "closing tag identifier",
                                found: other,
                                line: closing_token.span.line,
                                col: closing_token.span.col,
                            });
                        }
                    };

                    if closing_name != tag_name {
                        return Err(ParseError::MismatchedClosingTag {
                            expected: tag_name,
                            found: closing_name,
                            line: close_span.line,
                            col: close_span.col,
                        });
                    }

                    let gt_tok = self.lexer.next_token()?;
                    if gt_tok.kind != TokenKind::Greater {
                        return Err(ParseError::UnexpectedToken {
                            expected: ">",
                            found: gt_tok.kind,
                            line: gt_tok.span.line,
                            col: gt_tok.span.col,
                        });
                    }

                    // Synchronize parser token for parent
                    if self.tag_depth > 0 {
                        self.current = self.lexer.next_jsx_child()?;
                    } else {
                        self.current = self.lexer.next_token()?;
                    }

                    let end_span = gt_tok.span;
                    let span = Span::new(
                        start_span.start,
                        end_span.end,
                        start_span.line,
                        start_span.col,
                    );
                    return Ok(FsxElement {
                        tag_name,
                        attributes,
                        children,
                        span,
                    });
                }
                TokenKind::Eof => {
                    return Err(ParseError::UnexpectedEof("JSX closing tag"));
                }
                other => {
                    return Err(ParseError::UnexpectedToken {
                        expected: "JSX child (element, text, expression, or closing tag)",
                        found: other,
                        line: self.current.span.line,
                        col: self.current.span.col,
                    });
                }
            }
        }
    }

    // Expressions parsing with precedence
    pub fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_binary_expr(0)
    }

    fn parse_binary_expr(&mut self, min_prec: u8) -> Result<Expr, ParseError> {
        let mut left = self.parse_primary_or_postfix()?;

        while let Some((op, prec)) = self.current_binary_op() {
            if prec < min_prec {
                break;
            }
            self.advance()?; // consume op
            let right = self.parse_binary_expr(prec + 1)?;
            let span = Span::new(
                left.span().start,
                right.span().end,
                left.span().line,
                left.span().col,
            );
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
                span,
            };
        }

        Ok(left)
    }

    fn current_binary_op(&self) -> Option<(BinaryOp, u8)> {
        match self.current.kind {
            TokenKind::Plus => Some((BinaryOp::Add, 10)),
            TokenKind::Minus => Some((BinaryOp::Sub, 10)),
            TokenKind::Star => Some((BinaryOp::Mul, 20)),
            TokenKind::Slash => Some((BinaryOp::Div, 20)),
            _ => None,
        }
    }

    fn parse_primary_or_postfix(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.current.kind == TokenKind::Dot {
                self.advance()?; // consume '.'
                let (member, m_span) = self.expect_ident()?;
                let span = Span::new(
                    expr.span().start,
                    m_span.end,
                    expr.span().line,
                    expr.span().col,
                );
                expr = Expr::Member(Box::new(expr), member, span);
            } else if self.current.kind == TokenKind::OpenParen {
                self.advance()?; // consume '('
                let mut args = Vec::new();
                while self.current.kind != TokenKind::CloseParen
                    && self.current.kind != TokenKind::Eof
                {
                    args.push(self.parse_expr()?);
                    if self.current.kind == TokenKind::Comma {
                        self.advance()?;
                    } else {
                        break;
                    }
                }
                let end_span = self.expect(&TokenKind::CloseParen, ")")?.span;
                let span = Span::new(
                    expr.span().start,
                    end_span.end,
                    expr.span().line,
                    expr.span().col,
                );
                expr = Expr::Call(Box::new(expr), args, span);
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let span = self.current_span();
        match self.current.kind.clone() {
            TokenKind::Ident(id) => {
                self.advance()?;
                Ok(Expr::Ident(id, span))
            }
            TokenKind::Int(i) => {
                self.advance()?;
                Ok(Expr::Int(i, span))
            }
            TokenKind::Float(f) => {
                self.advance()?;
                Ok(Expr::Float(f, span))
            }
            TokenKind::StringLit(s) => {
                self.advance()?;
                Ok(Expr::StringLit(s, span))
            }
            TokenKind::True => {
                self.advance()?;
                Ok(Expr::Bool(true, span))
            }
            TokenKind::False => {
                self.advance()?;
                Ok(Expr::Bool(false, span))
            }
            TokenKind::Bang => {
                self.advance()?;
                let inner = self.parse_primary_or_postfix()?;
                let end_span = inner.span();
                let full_span = Span::new(span.start, end_span.end, span.line, span.col);
                Ok(Expr::Unary {
                    op: UnaryOp::Not,
                    expr: Box::new(inner),
                    span: full_span,
                })
            }
            TokenKind::OpenParen => {
                self.advance()?; // consume '('
                if self.current.kind == TokenKind::CloseParen {
                    self.advance()?; // consume ')'
                    self.expect(&TokenKind::FatArrow, "=>")?;
                    let body = self.parse_expr()?;
                    let end_span = body.span();
                    let fn_span = Span::new(span.start, end_span.end, span.line, span.col);
                    return Ok(Expr::ArrowFn {
                        params: Vec::new(),
                        body: Box::new(body),
                        span: fn_span,
                    });
                }
                let inner = self.parse_expr()?;
                self.expect(&TokenKind::CloseParen, ")")?;
                if self.current.kind == TokenKind::FatArrow {
                    self.advance()?; // consume '=>'
                    let body = self.parse_expr()?;
                    let end_span = body.span();
                    let fn_span = Span::new(span.start, end_span.end, span.line, span.col);
                    let param_name = match inner {
                        Expr::Ident(name, _) => name,
                        _ => "param".to_string(),
                    };
                    return Ok(Expr::ArrowFn {
                        params: vec![param_name],
                        body: Box::new(body),
                        span: fn_span,
                    });
                }
                Ok(inner)
            }
            _ => Err(ParseError::UnexpectedToken {
                expected: "expression",
                found: self.current.kind.clone(),
                line: span.line,
                col: span.col,
            }),
        }
    }
}

fn is_hook_name(s: &str) -> bool {
    matches!(
        s,
        "useState"
            | "useSignal"
            | "useMemo"
            | "useEffect"
            | "useRef"
            | "useContext"
            | "useResource"
    )
}

fn clean_jsx_text(text: &str) -> Option<String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }
    if !text.contains('\n') {
        Some(text.to_string())
    } else {
        Some(trimmed.to_string())
    }
}
