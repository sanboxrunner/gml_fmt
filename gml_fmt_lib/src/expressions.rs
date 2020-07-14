use super::lex_token::*;
use super::statements::DelimitedLines;
pub type ExprBox<'a> = Box<ExprBoxInterior<'a>>;
pub type CommentsAndNewlines<'a> = Option<Vec<Token<'a>>>;
pub type DSAccess<'a> = Vec<(CommentsAndNewlines<'a>, ExprBox<'a>)>;

#[derive(Debug)]
pub struct ExprBoxInterior<'a> {
    pub expr: Expr<'a>,
    pub trailing_comments: CommentsAndNewlines<'a>,
}

#[derive(Debug)]
pub enum Expr<'a> {
    Call {
        procedure_name: ExprBox<'a>,
        comments_and_newlines_after_lparen: CommentsAndNewlines<'a>,
        arguments: DelimitedLines<'a, ExprBox<'a>>,
        is_constructor: bool,
    },
    Binary {
        left: ExprBox<'a>,
        operator: Token<'a>,
        comments_and_newlines_between_op_and_r: CommentsAndNewlines<'a>,
        right: ExprBox<'a>,
    },
    Grouping {
        comments_and_newlines_after_lparen: CommentsAndNewlines<'a>,
        expressions: Vec<ExprBox<'a>>,
        comments_and_newlines_after_rparen: CommentsAndNewlines<'a>,
    },
    ArrayLiteral {
        comments_and_newlines_after_lbracket: CommentsAndNewlines<'a>,
        arguments: DelimitedLines<'a, ExprBox<'a>>,
    },
    Literal {
        literal_token: Token<'a>,
        comments: CommentsAndNewlines<'a>,
    },
    NumberStartDot {
        literal_token: Token<'a>,
        comments: CommentsAndNewlines<'a>,
    },
    NumberEndDot {
        literal_token: Token<'a>,
        comments: CommentsAndNewlines<'a>,
    },
    Unary {
        operator: Token<'a>,
        comments_and_newlines_between: CommentsAndNewlines<'a>,
        right: ExprBox<'a>,
    },
    Postfix {
        operator: Token<'a>,
        comments_and_newlines_between: CommentsAndNewlines<'a>,
        expr: ExprBox<'a>,
    },
    Assign {
        left: ExprBox<'a>,
        operator: Token<'a>,
        comments_and_newlines_between_op_and_r: CommentsAndNewlines<'a>,
        right: ExprBox<'a>,
        calls_constructor: bool,
    },
    Identifier {
        name: Token<'a>,
        comments: CommentsAndNewlines<'a>,
    },
    DotAccess {
        object_name: ExprBox<'a>,
        comments_between: CommentsAndNewlines<'a>,
        instance_variable: ExprBox<'a>,
    },
    DataStructureAccess {
        ds_name: ExprBox<'a>,
        access_type: Token<'a>,
        access_exprs: DSAccess<'a>,
    },
    // x ? y : z;
    Ternary {
        conditional: ExprBox<'a>,
        comments_and_newlines_after_q: CommentsAndNewlines<'a>,
        left: ExprBox<'a>,
        comments_and_newlines_after_colon: CommentsAndNewlines<'a>,
        right: ExprBox<'a>,
    },
    Newline,
    Comment {
        comment: Token<'a>,
    },
    MultilineComment {
        multiline_comment: Token<'a>,
    },
    UnidentifiedAsLiteral {
        literal_token: Token<'a>,
    },
}
