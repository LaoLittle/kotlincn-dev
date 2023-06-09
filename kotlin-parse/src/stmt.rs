use crate::stream::Token;
use crate::Parser;
use kotlin_ast::decl::{DeclStmt, VarKind, VariableDecl};
use kotlin_ast::expr::ExprStmt;
use kotlin_ast::stmt::{AssignStmt, ForStmt, Stmt, WhileStmt};

impl<'a> Parser<'a> {
    pub fn parse_stmt_list(&mut self) -> Vec<Stmt> {
        let mut stmts = vec![];
        while !matches!(self.peek_token_skip_nl(), Token::Eof | Token::CloseBrace) {
            stmts.push(self.parse_stmt());
        }

        stmts
    }

    pub fn parse_stmt(&mut self) -> Stmt {
        match self.advance_token_skip_nl() {
            Token::Package => Stmt::Decl(DeclStmt::Package(self.parse_package_decl())),
            Token::Import => Stmt::Decl(DeclStmt::Import(self.parse_import_decl())),
            Token::Fun => Stmt::Decl(DeclStmt::Fun(self.parse_fun_decl())),
            Token::Val => Stmt::Decl(DeclStmt::Variable(self.parse_variable_decl(false))),
            Token::Var => Stmt::Decl(DeclStmt::Variable(self.parse_variable_decl(true))),
            Token::While => Stmt::While(self.parse_while_stmt()),
            Token::For => Stmt::For(self.parse_for_stmt()),
            Token::Semi | Token::Eof => Stmt::Empty,
            tk => {
                self.lookahead = Some(tk);
                let expr = self.parse_expr();
                if let ExprStmt::Ident(id) = expr {
                    match self.peek_token() {
                        Token::Assign => {
                            self.bump();
                            let expr = self.parse_expr();
                            return Stmt::Assign(AssignStmt { id, expr });
                        }
                        _ => {}
                    }
                }

                Stmt::Expr(expr)
            }
        }
    }

    pub fn parse_while_stmt(&mut self) -> WhileStmt {
        self.expect_skip_nl(Token::OpenParen);
        let cond = self.parse_expr();
        self.expect_skip_nl(Token::CloseParen);

        let b = self.parse_block_even_single_expr();

        WhileStmt { cond, body: b }
    }

    pub fn parse_for_stmt(&mut self) -> ForStmt {
        self.expect_skip_nl(Token::OpenParen);
        self.expect_skip_nl(Token::Ident);
        let val = self.last_ident();
        self.expect_skip_nl(Token::In);
        let target = self.parse_expr();
        self.expect_skip_nl(Token::CloseParen);

        let body = self.parse_block_even_single_expr();

        ForStmt {
            bind: val,
            target,
            body,
        }
    }

    pub fn parse_variable_decl(&mut self, mutable: bool) -> VariableDecl {
        self.expect_skip_nl(Token::Ident);
        let name = self.last_ident();

        let ty = match self.peek_token_skip_nl() {
            Token::Colon => {
                self.bump();
                self.expect_skip_nl(Token::Ident);
                Some(self.last_ident())
            }
            _ => None,
        };

        let mut skipped = false;
        let kind = loop {
            match self.advance_token() {
                Token::NewLine => skipped = true,
                Token::Assign => break VarKind::Init(self.parse_expr()),
                tk => {
                    self.lookahead = Some(tk);
                    if !skipped {
                        self.expect(Token::Assign);
                    }

                    break VarKind::Decl;
                }
            }
        };

        VariableDecl {
            mutable,
            name,
            ty,
            kind,
        }
    }
}
