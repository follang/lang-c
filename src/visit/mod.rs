//! Recursive abstract syntax tree traversal
//!
//! ```rust
//! # use lang_c::{ast, span, visit};
//! struct ExprCount(usize);
//!
//! impl<'ast> visit::Visit<'ast> for ExprCount {
//!     fn visit_expression(&mut self, expr: &'ast ast::Expression, span: &'ast span::Span) {
//!         self.0 += 1;
//!         visit::visit_expression(self, expr, span);
//!     }
//! }
//! ```
//!
//! The `Visit` trait is a collection of hooks, one for each type of node in the AST (for each type
//! in the `ast` module). Default implementations will recursively visit the sub-nodes (by calling
//! a corresponding free function in this module).
//!
//! Free functions apply the visitor to sub-nodes of any given AST node.

use crate::ast::*;
use crate::span::Span;

mod declarations;
mod expressions;
mod extensions;
mod lexical;
mod statements;

pub use self::declarations::*;
pub use self::expressions::*;
pub use self::extensions::*;
pub use self::lexical::*;
pub use self::statements::*;

pub trait Visit<'ast> {
    fn visit_identifier(&mut self, identifier: &'ast Identifier, span: &'ast Span) {
        visit_identifier(self, identifier, span)
    }

    fn visit_constant(&mut self, constant: &'ast Constant, span: &'ast Span) {
        visit_constant(self, constant, span)
    }

    fn visit_integer(&mut self, integer: &'ast Integer, span: &'ast Span) {
        visit_integer(self, integer, span)
    }

    fn visit_integer_base(&mut self, integer_base: &'ast IntegerBase, span: &'ast Span) {
        visit_integer_base(self, integer_base, span)
    }

    fn visit_integer_suffix(&mut self, integer_suffix: &'ast IntegerSuffix, span: &'ast Span) {
        visit_integer_suffix(self, integer_suffix, span)
    }

    fn visit_integer_size(&mut self, integer_size: &'ast IntegerSize, span: &'ast Span) {
        visit_integer_size(self, integer_size, span)
    }

    fn visit_float(&mut self, float: &'ast Float, span: &'ast Span) {
        visit_float(self, float, span)
    }

    fn visit_float_base(&mut self, float_base: &'ast FloatBase, span: &'ast Span) {
        visit_float_base(self, float_base, span)
    }

    fn visit_float_suffix(&mut self, float_suffix: &'ast FloatSuffix, span: &'ast Span) {
        visit_float_suffix(self, float_suffix, span)
    }

    fn visit_float_format(&mut self, float_format: &'ast FloatFormat, span: &'ast Span) {
        visit_float_format(self, float_format, span)
    }

    fn visit_string_literal(&mut self, string_literal: &'ast StringLiteral, span: &'ast Span) {
        visit_string_literal(self, string_literal, span)
    }

    fn visit_expression(&mut self, expression: &'ast Expression, span: &'ast Span) {
        visit_expression(self, expression, span)
    }

    fn visit_member_operator(&mut self, member_operator: &'ast MemberOperator, span: &'ast Span) {
        visit_member_operator(self, member_operator, span)
    }

    fn visit_generic_selection(
        &mut self,
        generic_selection: &'ast GenericSelection,
        span: &'ast Span,
    ) {
        visit_generic_selection(self, generic_selection, span)
    }

    fn visit_generic_association(
        &mut self,
        generic_association: &'ast GenericAssociation,
        span: &'ast Span,
    ) {
        visit_generic_association(self, generic_association, span)
    }

    fn visit_generic_association_type(
        &mut self,
        generic_association_type: &'ast GenericAssociationType,
        span: &'ast Span,
    ) {
        visit_generic_association_type(self, generic_association_type, span)
    }

    fn visit_member_expression(
        &mut self,
        member_expression: &'ast MemberExpression,
        span: &'ast Span,
    ) {
        visit_member_expression(self, member_expression, span)
    }

    fn visit_call_expression(&mut self, call_expression: &'ast CallExpression, span: &'ast Span) {
        visit_call_expression(self, call_expression, span)
    }

    fn visit_compound_literal(
        &mut self,
        compound_literal: &'ast CompoundLiteral,
        span: &'ast Span,
    ) {
        visit_compound_literal(self, compound_literal, span)
    }

    fn visit_sizeofty(&mut self, sizeofty: &'ast SizeOfTy, span: &'ast Span) {
        visit_sizeofty(self, sizeofty, span)
    }

    fn visit_sizeofval(&mut self, sizeofval: &'ast SizeOfVal, span: &'ast Span) {
        visit_sizeofval(self, sizeofval, span)
    }

    fn visit_alignof(&mut self, alignofty: &'ast AlignOf, span: &'ast Span) {
        visit_alignof(self, alignofty, span)
    }

    fn visit_unary_operator(&mut self, unary_operator: &'ast UnaryOperator, span: &'ast Span) {
        visit_unary_operator(self, unary_operator, span)
    }

    fn visit_unary_operator_expression(
        &mut self,
        unary_operator_expression: &'ast UnaryOperatorExpression,
        span: &'ast Span,
    ) {
        visit_unary_operator_expression(self, unary_operator_expression, span)
    }

    fn visit_cast_expression(&mut self, cast_expression: &'ast CastExpression, span: &'ast Span) {
        visit_cast_expression(self, cast_expression, span)
    }

    fn visit_binary_operator(&mut self, binary_operator: &'ast BinaryOperator, span: &'ast Span) {
        visit_binary_operator(self, binary_operator, span)
    }

    fn visit_binary_operator_expression(
        &mut self,
        binary_operator_expression: &'ast BinaryOperatorExpression,
        span: &'ast Span,
    ) {
        visit_binary_operator_expression(self, binary_operator_expression, span)
    }

    fn visit_conditional_expression(
        &mut self,
        conditional_expression: &'ast ConditionalExpression,
        span: &'ast Span,
    ) {
        visit_conditional_expression(self, conditional_expression, span)
    }

    fn visit_va_arg_expression(
        &mut self,
        va_arg_expression: &'ast VaArgExpression,
        span: &'ast Span,
    ) {
        visit_va_arg_expression(self, va_arg_expression, span)
    }

    fn visit_offset_of_expression(
        &mut self,
        offset_of_expression: &'ast OffsetOfExpression,
        span: &'ast Span,
    ) {
        visit_offset_of_expression(self, offset_of_expression, span)
    }

    fn visit_offset_designator(
        &mut self,
        offset_designator: &'ast OffsetDesignator,
        span: &'ast Span,
    ) {
        visit_offset_designator(self, offset_designator, span)
    }

    fn visit_offset_member(&mut self, offset_member: &'ast OffsetMember, span: &'ast Span) {
        visit_offset_member(self, offset_member, span)
    }

    fn visit_declaration(&mut self, declaration: &'ast Declaration, span: &'ast Span) {
        visit_declaration(self, declaration, span)
    }

    fn visit_declaration_specifier(
        &mut self,
        declaration_specifier: &'ast DeclarationSpecifier,
        span: &'ast Span,
    ) {
        visit_declaration_specifier(self, declaration_specifier, span)
    }

    fn visit_init_declarator(&mut self, init_declarator: &'ast InitDeclarator, span: &'ast Span) {
        visit_init_declarator(self, init_declarator, span)
    }

    fn visit_storage_class_specifier(
        &mut self,
        storage_class_specifier: &'ast StorageClassSpecifier,
        span: &'ast Span,
    ) {
        visit_storage_class_specifier(self, storage_class_specifier, span)
    }

    fn visit_type_specifier(&mut self, type_specifier: &'ast TypeSpecifier, span: &'ast Span) {
        visit_type_specifier(self, type_specifier, span)
    }

    fn visit_ts18661_float_type(
        &mut self,
        ts18661_float_type: &'ast TS18661FloatType,
        span: &'ast Span,
    ) {
        visit_ts18661_float_type(self, ts18661_float_type, span)
    }

    fn visit_ts18661_float_format(
        &mut self,
        ts18661_float_format: &'ast TS18661FloatFormat,
        span: &'ast Span,
    ) {
        visit_ts18661_float_format(self, ts18661_float_format, span)
    }

    fn visit_struct_type(&mut self, struct_type: &'ast StructType, span: &'ast Span) {
        visit_struct_type(self, struct_type, span)
    }

    fn visit_struct_kind(&mut self, struct_kind: &'ast StructKind, span: &'ast Span) {
        visit_struct_kind(self, struct_kind, span)
    }

    fn visit_struct_declaration(
        &mut self,
        struct_declaration: &'ast StructDeclaration,
        span: &'ast Span,
    ) {
        visit_struct_declaration(self, struct_declaration, span)
    }

    fn visit_struct_field(&mut self, struct_field: &'ast StructField, span: &'ast Span) {
        visit_struct_field(self, struct_field, span)
    }

    fn visit_specifier_qualifier(
        &mut self,
        specifier_qualifier: &'ast SpecifierQualifier,
        span: &'ast Span,
    ) {
        visit_specifier_qualifier(self, specifier_qualifier, span)
    }

    fn visit_struct_declarator(
        &mut self,
        struct_declarator: &'ast StructDeclarator,
        span: &'ast Span,
    ) {
        visit_struct_declarator(self, struct_declarator, span)
    }

    fn visit_enum_type(&mut self, enum_type: &'ast EnumType, span: &'ast Span) {
        visit_enum_type(self, enum_type, span)
    }

    fn visit_enumerator(&mut self, enumerator: &'ast Enumerator, span: &'ast Span) {
        visit_enumerator(self, enumerator, span)
    }

    fn visit_type_qualifier(&mut self, type_qualifier: &'ast TypeQualifier, span: &'ast Span) {
        visit_type_qualifier(self, type_qualifier, span)
    }

    fn visit_function_specifier(
        &mut self,
        function_specifier: &'ast FunctionSpecifier,
        span: &'ast Span,
    ) {
        visit_function_specifier(self, function_specifier, span)
    }

    fn visit_alignment_specifier(
        &mut self,
        alignment_specifier: &'ast AlignmentSpecifier,
        span: &'ast Span,
    ) {
        visit_alignment_specifier(self, alignment_specifier, span)
    }

    fn visit_declarator(&mut self, declarator: &'ast Declarator, span: &'ast Span) {
        visit_declarator(self, declarator, span)
    }

    fn visit_declarator_kind(&mut self, declarator_kind: &'ast DeclaratorKind, span: &'ast Span) {
        visit_declarator_kind(self, declarator_kind, span)
    }

    fn visit_derived_declarator(
        &mut self,
        derived_declarator: &'ast DerivedDeclarator,
        span: &'ast Span,
    ) {
        visit_derived_declarator(self, derived_declarator, span)
    }

    fn visit_array_declarator(
        &mut self,
        array_declarator: &'ast ArrayDeclarator,
        span: &'ast Span,
    ) {
        visit_array_declarator(self, array_declarator, span)
    }

    fn visit_function_declarator(
        &mut self,
        function_declarator: &'ast FunctionDeclarator,
        span: &'ast Span,
    ) {
        visit_function_declarator(self, function_declarator, span)
    }

    fn visit_pointer_qualifier(
        &mut self,
        pointer_qualifier: &'ast PointerQualifier,
        span: &'ast Span,
    ) {
        visit_pointer_qualifier(self, pointer_qualifier, span)
    }

    fn visit_array_size(&mut self, array_size: &'ast ArraySize, span: &'ast Span) {
        visit_array_size(self, array_size, span)
    }

    fn visit_parameter_declaration(
        &mut self,
        parameter_declaration: &'ast ParameterDeclaration,
        span: &'ast Span,
    ) {
        visit_parameter_declaration(self, parameter_declaration, span)
    }

    fn visit_ellipsis(&mut self, ellipsis: &'ast Ellipsis, span: &'ast Span) {
        visit_ellipsis(self, ellipsis, span)
    }

    fn visit_type_name(&mut self, type_name: &'ast TypeName, span: &'ast Span) {
        visit_type_name(self, type_name, span)
    }

    fn visit_initializer(&mut self, initializer: &'ast Initializer, span: &'ast Span) {
        visit_initializer(self, initializer, span)
    }

    fn visit_initializer_list_item(
        &mut self,
        initializer_list_item: &'ast InitializerListItem,
        span: &'ast Span,
    ) {
        visit_initializer_list_item(self, initializer_list_item, span)
    }

    fn visit_designator(&mut self, designator: &'ast Designator, span: &'ast Span) {
        visit_designator(self, designator, span)
    }

    fn visit_range_designator(
        &mut self,
        range_designator: &'ast RangeDesignator,
        span: &'ast Span,
    ) {
        visit_range_designator(self, range_designator, span)
    }

    fn visit_static_assert(&mut self, static_assert: &'ast StaticAssert, span: &'ast Span) {
        visit_static_assert(self, static_assert, span)
    }

    fn visit_statement(&mut self, statement: &'ast Statement, span: &'ast Span) {
        visit_statement(self, statement, span)
    }

    fn visit_labeled_statement(
        &mut self,
        labeled_statement: &'ast LabeledStatement,
        span: &'ast Span,
    ) {
        visit_labeled_statement(self, labeled_statement, span)
    }

    fn visit_if_statement(&mut self, if_statement: &'ast IfStatement, span: &'ast Span) {
        visit_if_statement(self, if_statement, span)
    }

    fn visit_switch_statement(
        &mut self,
        switch_statement: &'ast SwitchStatement,
        span: &'ast Span,
    ) {
        visit_switch_statement(self, switch_statement, span)
    }

    fn visit_while_statement(&mut self, while_statement: &'ast WhileStatement, span: &'ast Span) {
        visit_while_statement(self, while_statement, span)
    }

    fn visit_do_while_statement(
        &mut self,
        do_while_statement: &'ast DoWhileStatement,
        span: &'ast Span,
    ) {
        visit_do_while_statement(self, do_while_statement, span)
    }

    fn visit_for_statement(&mut self, for_statement: &'ast ForStatement, span: &'ast Span) {
        visit_for_statement(self, for_statement, span)
    }

    fn visit_label(&mut self, label: &'ast Label, span: &'ast Span) {
        visit_label(self, label, span)
    }

    fn visit_case_range(&mut self, range: &'ast CaseRange, span: &'ast Span) {
        visit_case_range(self, range, span)
    }

    fn visit_for_initializer(&mut self, for_initializer: &'ast ForInitializer, span: &'ast Span) {
        visit_for_initializer(self, for_initializer, span)
    }

    fn visit_block_item(&mut self, block_item: &'ast BlockItem, span: &'ast Span) {
        visit_block_item(self, block_item, span)
    }

    fn visit_translation_unit(&mut self, translation_unit: &'ast TranslationUnit) {
        visit_translation_unit(self, translation_unit)
    }

    fn visit_external_declaration(
        &mut self,
        external_declaration: &'ast ExternalDeclaration,
        span: &'ast Span,
    ) {
        visit_external_declaration(self, external_declaration, span)
    }

    fn visit_function_definition(
        &mut self,
        function_definition: &'ast FunctionDefinition,
        span: &'ast Span,
    ) {
        visit_function_definition(self, function_definition, span)
    }

    fn visit_extension(&mut self, extension: &'ast Extension, span: &'ast Span) {
        visit_extension(self, extension, span)
    }

    fn visit_attribute(&mut self, attribute: &'ast Attribute, span: &'ast Span) {
        visit_attribute(self, attribute, span)
    }

    fn visit_asm_statement(&mut self, asm_statement: &'ast AsmStatement, span: &'ast Span) {
        visit_asm_statement(self, asm_statement, span)
    }

    fn visit_availability_attribute(
        &mut self,
        availability: &'ast AvailabilityAttribute,
        span: &'ast Span,
    ) {
        visit_availability_attribute(self, availability, span)
    }

    fn visit_availability_clause(&mut self, _clause: &'ast AvailabilityClause, _span: &'ast Span) {}

    fn visit_gnu_extended_asm_statement(
        &mut self,
        gnu_extended_asm_statement: &'ast GnuExtendedAsmStatement,
        span: &'ast Span,
    ) {
        visit_gnu_extended_asm_statement(self, gnu_extended_asm_statement, span)
    }

    fn visit_gnu_asm_operand(&mut self, gnu_asm_operand: &'ast GnuAsmOperand, span: &'ast Span) {
        visit_gnu_asm_operand(self, gnu_asm_operand, span)
    }

    fn visit_type_of(&mut self, type_of: &'ast TypeOf, span: &'ast Span) {
        visit_type_of(self, type_of, span)
    }
}
