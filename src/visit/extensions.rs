use crate::ast::*;
use crate::span::Span;

use super::Visit;

pub fn visit_extension<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    extension: &'ast Extension,
    span: &'ast Span,
) {
    match *extension {
        Extension::Attribute(ref a) => visitor.visit_attribute(a, span),
        Extension::AsmLabel(ref a) => visitor.visit_string_literal(&a.node, &a.span),
        Extension::AvailabilityAttribute(ref a) => visitor.visit_availability_attribute(&a.node, &a.span),
    }
}

pub fn visit_attribute<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    attribute: &'ast Attribute,
    _span: &'ast Span,
) {
    for argument in &attribute.arguments {
        visitor.visit_expression(&argument.node, &argument.span);
    }
}

pub fn visit_asm_statement<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    asm_statement: &'ast AsmStatement,
    span: &'ast Span,
) {
    match *asm_statement {
        AsmStatement::GnuBasic(ref g) => visitor.visit_string_literal(&g.node, &g.span),
        AsmStatement::GnuExtended(ref g) => visitor.visit_gnu_extended_asm_statement(g, span),
    }
}

pub fn visit_availability_attribute<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    availability: &'ast AvailabilityAttribute,
    _span: &'ast Span,
) {
    for clause in &availability.clauses {
        visitor.visit_availability_clause(&clause.node, &clause.span);
    }
}

pub fn visit_gnu_extended_asm_statement<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    gnu_extended_asm_statement: &'ast GnuExtendedAsmStatement,
    _span: &'ast Span,
) {
    if let Some(ref qualifier) = gnu_extended_asm_statement.qualifier {
        visitor.visit_type_qualifier(&qualifier.node, &qualifier.span);
    }
    visitor.visit_string_literal(&gnu_extended_asm_statement.template.node, &gnu_extended_asm_statement.template.span);
    for output in &gnu_extended_asm_statement.outputs {
        visitor.visit_gnu_asm_operand(&output.node, &output.span);
    }
    for input in &gnu_extended_asm_statement.inputs {
        visitor.visit_gnu_asm_operand(&input.node, &input.span);
    }
    for clobber in &gnu_extended_asm_statement.clobbers {
        visitor.visit_string_literal(&clobber.node, &clobber.span);
    }
}

pub fn visit_gnu_asm_operand<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    gnu_asm_operand: &'ast GnuAsmOperand,
    _span: &'ast Span,
) {
    if let Some(ref name) = gnu_asm_operand.symbolic_name {
        visitor.visit_identifier(&name.node, &name.span);
    }
    visitor.visit_string_literal(&gnu_asm_operand.constraints.node, &gnu_asm_operand.constraints.span);
    visitor.visit_expression(&gnu_asm_operand.variable_name.node, &gnu_asm_operand.variable_name.span);
}

pub fn visit_type_of<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    type_of: &'ast TypeOf,
    _span: &'ast Span,
) {
    match *type_of {
        TypeOf::Expression(ref e) => visitor.visit_expression(&e.node, &e.span),
        TypeOf::Type(ref t) => visitor.visit_type_name(&t.node, &t.span),
    }
}
