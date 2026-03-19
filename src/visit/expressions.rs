use crate::ast::*;
use crate::span::Span;

use super::Visit;

pub fn visit_expression<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    expression: &'ast Expression,
    _span: &'ast Span,
) {
    match *expression {
        Expression::Identifier(ref i) => visitor.visit_identifier(&i.node, &i.span),
        Expression::Constant(ref c) => visitor.visit_constant(&c.node, &c.span),
        Expression::StringLiteral(ref s) => visitor.visit_string_literal(&s.node, &s.span),
        Expression::GenericSelection(ref g) => visitor.visit_generic_selection(&g.node, &g.span),
        Expression::Member(ref m) => visitor.visit_member_expression(&m.node, &m.span),
        Expression::Call(ref c) => visitor.visit_call_expression(&c.node, &c.span),
        Expression::CompoundLiteral(ref c) => visitor.visit_compound_literal(&c.node, &c.span),
        Expression::SizeOfTy(ref s) => visitor.visit_sizeofty(&s.node, &s.span),
        Expression::SizeOfVal(ref s) => visitor.visit_sizeofval(&s.node, &s.span),
        Expression::AlignOf(ref a) => visitor.visit_alignof(&a.node, &a.span),
        Expression::UnaryOperator(ref u) => visitor.visit_unary_operator_expression(&u.node, &u.span),
        Expression::Cast(ref c) => visitor.visit_cast_expression(&c.node, &c.span),
        Expression::BinaryOperator(ref b) => {
            visitor.visit_binary_operator_expression(&b.node, &b.span)
        }
        Expression::Conditional(ref c) => visitor.visit_conditional_expression(&c.node, &c.span),
        Expression::Comma(ref comma) => {
            for c in comma.iter() {
                visitor.visit_expression(&c.node, &c.span);
            }
        }
        Expression::OffsetOf(ref o) => visitor.visit_offset_of_expression(&o.node, &o.span),
        Expression::VaArg(ref v) => visitor.visit_va_arg_expression(&v.node, &v.span),
        Expression::Statement(ref s) => visitor.visit_statement(&s.node, &s.span),
    }
}

pub fn visit_member_operator<'ast, V: Visit<'ast> + ?Sized>(
    _visitor: &mut V,
    _member_operator: &'ast MemberOperator,
    _span: &'ast Span,
) {
}

pub fn visit_generic_selection<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    generic_selection: &'ast GenericSelection,
    _span: &'ast Span,
) {
    visitor.visit_expression(&generic_selection.expression.node, &generic_selection.expression.span);
    for association in &generic_selection.associations {
        visitor.visit_generic_association(&association.node, &association.span);
    }
}

pub fn visit_generic_association<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    generic_association: &'ast GenericAssociation,
    _span: &'ast Span,
) {
    match *generic_association {
        GenericAssociation::Type(ref t) => visitor.visit_generic_association_type(&t.node, &t.span),
        GenericAssociation::Default(ref d) => visitor.visit_expression(&d.node, &d.span),
    }
}

pub fn visit_generic_association_type<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    generic_association_type: &'ast GenericAssociationType,
    _span: &'ast Span,
) {
    visitor.visit_type_name(&generic_association_type.type_name.node, &generic_association_type.type_name.span);
    visitor.visit_expression(&generic_association_type.expression.node, &generic_association_type.expression.span);
}

pub fn visit_member_expression<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    member_expression: &'ast MemberExpression,
    _span: &'ast Span,
) {
    visitor.visit_member_operator(&member_expression.operator.node, &member_expression.operator.span);
    visitor.visit_expression(&member_expression.expression.node, &member_expression.expression.span);
    visitor.visit_identifier(&member_expression.identifier.node, &member_expression.identifier.span);
}

pub fn visit_call_expression<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    call_expression: &'ast CallExpression,
    _span: &'ast Span,
) {
    visitor.visit_expression(&call_expression.callee.node, &call_expression.callee.span);
    for argument in &call_expression.arguments {
        visitor.visit_expression(&argument.node, &argument.span);
    }
}

pub fn visit_compound_literal<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    compound_literal: &'ast CompoundLiteral,
    _span: &'ast Span,
) {
    visitor.visit_type_name(&compound_literal.type_name.node, &compound_literal.type_name.span);
    for initializer in &compound_literal.initializer_list {
        visitor.visit_initializer_list_item(&initializer.node, &initializer.span);
    }
}

pub fn visit_sizeofty<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    sizeofty: &'ast SizeOfTy,
    _span: &'ast Span,
) {
    visitor.visit_type_name(&sizeofty.0.node, &sizeofty.0.span);
}

pub fn visit_sizeofval<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    sizeofval: &'ast SizeOfVal,
    _span: &'ast Span,
) {
    visitor.visit_expression(&sizeofval.0.node, &sizeofval.0.span);
}

pub fn visit_alignof<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    alignofty: &'ast AlignOf,
    _span: &'ast Span,
) {
    visitor.visit_type_name(&alignofty.0.node, &alignofty.0.span);
}

pub fn visit_unary_operator<'ast, V: Visit<'ast> + ?Sized>(
    _visitor: &mut V,
    _unary_operator: &'ast UnaryOperator,
    _span: &'ast Span,
) {
}

pub fn visit_unary_operator_expression<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    unary_operator_expression: &'ast UnaryOperatorExpression,
    _span: &'ast Span,
) {
    match unary_operator_expression.operator.node {
        UnaryOperator::PostIncrement | UnaryOperator::PostDecrement => {
            visitor.visit_expression(&unary_operator_expression.operand.node, &unary_operator_expression.operand.span);
            visitor.visit_unary_operator(&unary_operator_expression.operator.node, &unary_operator_expression.operator.span);
        }
        _ => {
            visitor.visit_unary_operator(&unary_operator_expression.operator.node, &unary_operator_expression.operator.span);
            visitor.visit_expression(&unary_operator_expression.operand.node, &unary_operator_expression.operand.span);
        }
    }
}

pub fn visit_cast_expression<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    cast_expression: &'ast CastExpression,
    _span: &'ast Span,
) {
    visitor.visit_type_name(&cast_expression.type_name.node, &cast_expression.type_name.span);
    visitor.visit_expression(&cast_expression.expression.node, &cast_expression.expression.span);
}

pub fn visit_binary_operator<'ast, V: Visit<'ast> + ?Sized>(
    _visitor: &mut V,
    _binary_operator: &'ast BinaryOperator,
    _span: &'ast Span,
) {
}

pub fn visit_binary_operator_expression<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    binary_operator_expression: &'ast BinaryOperatorExpression,
    _span: &'ast Span,
) {
    visitor.visit_expression(&binary_operator_expression.lhs.node, &binary_operator_expression.lhs.span);
    visitor.visit_expression(&binary_operator_expression.rhs.node, &binary_operator_expression.rhs.span);
    visitor.visit_binary_operator(&binary_operator_expression.operator.node, &binary_operator_expression.operator.span);
}

pub fn visit_conditional_expression<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    conditional_expression: &'ast ConditionalExpression,
    _span: &'ast Span,
) {
    visitor.visit_expression(&conditional_expression.condition.node, &conditional_expression.condition.span);
    visitor.visit_expression(&conditional_expression.then_expression.node, &conditional_expression.then_expression.span);
    visitor.visit_expression(&conditional_expression.else_expression.node, &conditional_expression.else_expression.span);
}

pub fn visit_va_arg_expression<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    va_arg_expression: &'ast VaArgExpression,
    _span: &'ast Span,
) {
    visitor.visit_expression(&va_arg_expression.va_list.node, &va_arg_expression.va_list.span);
    visitor.visit_type_name(&va_arg_expression.type_name.node, &va_arg_expression.type_name.span);
}

pub fn visit_offset_of_expression<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    offset_of_expression: &'ast OffsetOfExpression,
    _span: &'ast Span,
) {
    visitor.visit_type_name(&offset_of_expression.type_name.node, &offset_of_expression.type_name.span);
    visitor.visit_offset_designator(&offset_of_expression.designator.node, &offset_of_expression.designator.span);
}

pub fn visit_offset_designator<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    offset_designator: &'ast OffsetDesignator,
    _span: &'ast Span,
) {
    visitor.visit_identifier(&offset_designator.base.node, &offset_designator.base.span);
    for member in &offset_designator.members {
        visitor.visit_offset_member(&member.node, &member.span);
    }
}

pub fn visit_offset_member<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    offset_member: &'ast OffsetMember,
    _span: &'ast Span,
) {
    match *offset_member {
        OffsetMember::Member(ref m) => visitor.visit_identifier(&m.node, &m.span),
        OffsetMember::IndirectMember(ref m) => visitor.visit_identifier(&m.node, &m.span),
        OffsetMember::Index(ref i) => visitor.visit_expression(&i.node, &i.span),
    }
}
