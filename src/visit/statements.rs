use crate::ast::*;
use crate::span::Span;

use super::Visit;

pub fn visit_statement<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    statement: &'ast Statement,
    _span: &'ast Span,
) {
    match *statement {
        Statement::Labeled(ref l) => visitor.visit_labeled_statement(&l.node, &l.span),
        Statement::Compound(ref c) => {
            for item in c {
                visitor.visit_block_item(&item.node, &item.span);
            }
        }
        Statement::Expression(Some(ref e)) => visitor.visit_expression(&e.node, &e.span),
        Statement::If(ref i) => visitor.visit_if_statement(&i.node, &i.span),
        Statement::Switch(ref s) => visitor.visit_switch_statement(&s.node, &s.span),
        Statement::While(ref w) => visitor.visit_while_statement(&w.node, &w.span),
        Statement::DoWhile(ref d) => visitor.visit_do_while_statement(&d.node, &d.span),
        Statement::For(ref f) => visitor.visit_for_statement(&f.node, &f.span),
        Statement::Goto(ref g) => visitor.visit_identifier(&g.node, &g.span),
        Statement::Return(Some(ref r)) => visitor.visit_expression(&r.node, &r.span),
        Statement::Asm(ref a) => visitor.visit_asm_statement(&a.node, &a.span),
        _ => {}
    }
}

pub fn visit_labeled_statement<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    labeled_statement: &'ast LabeledStatement,
    _span: &'ast Span,
) {
    visitor.visit_label(&labeled_statement.label.node, &labeled_statement.label.span);
    visitor.visit_statement(&labeled_statement.statement.node, &labeled_statement.statement.span);
}

pub fn visit_if_statement<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    if_statement: &'ast IfStatement,
    _span: &'ast Span,
) {
    visitor.visit_expression(&if_statement.condition.node, &if_statement.condition.span);
    visitor.visit_statement(&if_statement.then_statement.node, &if_statement.then_statement.span);
    if let Some(ref e) = if_statement.else_statement {
        visitor.visit_statement(&e.node, &e.span);
    }
}

pub fn visit_switch_statement<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    switch_statement: &'ast SwitchStatement,
    _span: &'ast Span,
) {
    visitor.visit_expression(&switch_statement.expression.node, &switch_statement.expression.span);
    visitor.visit_statement(&switch_statement.statement.node, &switch_statement.statement.span);
}

pub fn visit_while_statement<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    while_statement: &'ast WhileStatement,
    _span: &'ast Span,
) {
    visitor.visit_expression(&while_statement.expression.node, &while_statement.expression.span);
    visitor.visit_statement(&while_statement.statement.node, &while_statement.statement.span);
}

pub fn visit_do_while_statement<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    do_while_statement: &'ast DoWhileStatement,
    _span: &'ast Span,
) {
    visitor.visit_statement(&do_while_statement.statement.node, &do_while_statement.statement.span);
    visitor.visit_expression(&do_while_statement.expression.node, &do_while_statement.expression.span);
}

pub fn visit_for_statement<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    for_statement: &'ast ForStatement,
    _span: &'ast Span,
) {
    visitor.visit_for_initializer(&for_statement.initializer.node, &for_statement.initializer.span);
    if let Some(ref c) = for_statement.condition {
        visitor.visit_expression(&c.node, &c.span);
    }
    if let Some(ref s) = for_statement.step {
        visitor.visit_expression(&s.node, &s.span);
    }
    visitor.visit_statement(&for_statement.statement.node, &for_statement.statement.span);
}

pub fn visit_label<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    label: &'ast Label,
    _span: &'ast Span,
) {
    match *label {
        Label::Identifier(ref i) => visitor.visit_identifier(&i.node, &i.span),
        Label::Case(ref c) => visitor.visit_expression(&c.node, &c.span),
        Label::CaseRange(ref c) => visitor.visit_case_range(&c.node, &c.span),
        Label::Default => {}
    }
}

pub fn visit_case_range<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    range: &'ast CaseRange,
    _span: &'ast Span,
) {
    visitor.visit_expression(&range.low.node, &range.low.span);
    visitor.visit_expression(&range.high.node, &range.high.span);
}

pub fn visit_for_initializer<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    for_initializer: &'ast ForInitializer,
    _span: &'ast Span,
) {
    match *for_initializer {
        ForInitializer::Empty => {}
        ForInitializer::Expression(ref e) => visitor.visit_expression(&e.node, &e.span),
        ForInitializer::Declaration(ref d) => visitor.visit_declaration(&d.node, &d.span),
        ForInitializer::StaticAssert(ref s) => visitor.visit_static_assert(&s.node, &s.span),
    }
}

pub fn visit_block_item<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    block_item: &'ast BlockItem,
    _span: &'ast Span,
) {
    match *block_item {
        BlockItem::Declaration(ref d) => visitor.visit_declaration(&d.node, &d.span),
        BlockItem::StaticAssert(ref s) => visitor.visit_static_assert(&s.node, &s.span),
        BlockItem::Statement(ref s) => visitor.visit_statement(&s.node, &s.span),
    }
}
