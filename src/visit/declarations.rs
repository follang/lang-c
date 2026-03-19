use crate::ast::*;
use crate::span::Span;

use super::Visit;

pub fn visit_declaration<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    declaration: &'ast Declaration,
    _span: &'ast Span,
) {
    for specifier in &declaration.specifiers {
        visitor.visit_declaration_specifier(&specifier.node, &specifier.span);
    }
    for declarator in &declaration.declarators {
        visitor.visit_init_declarator(&declarator.node, &declarator.span);
    }
}

pub fn visit_declaration_specifier<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    declaration_specifier: &'ast DeclarationSpecifier,
    _span: &'ast Span,
) {
    match *declaration_specifier {
        DeclarationSpecifier::StorageClass(ref s) => visitor.visit_storage_class_specifier(&s.node, &s.span),
        DeclarationSpecifier::TypeSpecifier(ref t) => visitor.visit_type_specifier(&t.node, &t.span),
        DeclarationSpecifier::TypeQualifier(ref t) => visitor.visit_type_qualifier(&t.node, &t.span),
        DeclarationSpecifier::Function(ref f) => visitor.visit_function_specifier(&f.node, &f.span),
        DeclarationSpecifier::Alignment(ref a) => visitor.visit_alignment_specifier(&a.node, &a.span),
        DeclarationSpecifier::Extension(ref e) => {
            for extension in e {
                visitor.visit_extension(&extension.node, &extension.span);
            }
        }
    }
}

pub fn visit_init_declarator<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    init_declarator: &'ast InitDeclarator,
    _span: &'ast Span,
) {
    visitor.visit_declarator(&init_declarator.declarator.node, &init_declarator.declarator.span);
    if let Some(ref initializer) = init_declarator.initializer {
        visitor.visit_initializer(&initializer.node, &initializer.span);
    }
}

pub fn visit_storage_class_specifier<'ast, V: Visit<'ast> + ?Sized>(
    _visitor: &mut V,
    _storage_class_specifier: &'ast StorageClassSpecifier,
    _span: &'ast Span,
) {
}

pub fn visit_type_specifier<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    type_specifier: &'ast TypeSpecifier,
    span: &'ast Span,
) {
    match *type_specifier {
        TypeSpecifier::Atomic(ref a) => visitor.visit_type_name(&a.node, &a.span),
        TypeSpecifier::Struct(ref s) => visitor.visit_struct_type(&s.node, &s.span),
        TypeSpecifier::Enum(ref e) => visitor.visit_enum_type(&e.node, &e.span),
        TypeSpecifier::TypedefName(ref t) => visitor.visit_identifier(&t.node, &t.span),
        TypeSpecifier::TypeOf(ref t) => visitor.visit_type_of(&t.node, &t.span),
        TypeSpecifier::TS18661Float(ref t) => visitor.visit_ts18661_float_type(t, span),
        _ => {}
    }
}

pub fn visit_ts18661_float_type<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    ts18661_float_type: &'ast TS18661FloatType,
    span: &'ast Span,
) {
    visitor.visit_ts18661_float_format(&ts18661_float_type.format, span);
}

pub fn visit_ts18661_float_format<'ast, V: Visit<'ast> + ?Sized>(
    _visitor: &mut V,
    _ts18661_float_format: &'ast TS18661FloatFormat,
    _span: &'ast Span,
) {
}

pub fn visit_struct_type<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    struct_type: &'ast StructType,
    _span: &'ast Span,
) {
    visitor.visit_struct_kind(&struct_type.kind.node, &struct_type.kind.span);
    if let Some(ref identifier) = struct_type.identifier {
        visitor.visit_identifier(&identifier.node, &identifier.span);
    }
    if let Some(ref declarations) = struct_type.declarations {
        for declaration in declarations {
            visitor.visit_struct_declaration(&declaration.node, &declaration.span);
        }
    }
}

pub fn visit_struct_kind<'ast, V: Visit<'ast> + ?Sized>(
    _visitor: &mut V,
    _struct_kind: &'ast StructKind,
    _span: &'ast Span,
) {
}

pub fn visit_struct_declaration<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    struct_declaration: &'ast StructDeclaration,
    _span: &'ast Span,
) {
    match *struct_declaration {
        StructDeclaration::Field(ref f) => visitor.visit_struct_field(&f.node, &f.span),
        StructDeclaration::StaticAssert(ref s) => visitor.visit_static_assert(&s.node, &s.span),
    }
}

pub fn visit_struct_field<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    struct_field: &'ast StructField,
    _span: &'ast Span,
) {
    for specifier in &struct_field.specifiers {
        visitor.visit_specifier_qualifier(&specifier.node, &specifier.span);
    }
    for declarator in &struct_field.declarators {
        visitor.visit_struct_declarator(&declarator.node, &declarator.span);
    }
}

pub fn visit_specifier_qualifier<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    specifier_qualifier: &'ast SpecifierQualifier,
    _span: &'ast Span,
) {
    match *specifier_qualifier {
        SpecifierQualifier::TypeSpecifier(ref t) => visitor.visit_type_specifier(&t.node, &t.span),
        SpecifierQualifier::TypeQualifier(ref t) => visitor.visit_type_qualifier(&t.node, &t.span),
        SpecifierQualifier::Extension(ref e) => {
            for n in e {
                visitor.visit_extension(&n.node, &n.span);
            }
        }
    }
}

pub fn visit_struct_declarator<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    struct_declarator: &'ast StructDeclarator,
    _span: &'ast Span,
) {
    if let Some(ref declarator) = struct_declarator.declarator {
        visitor.visit_declarator(&declarator.node, &declarator.span);
    }
    if let Some(ref bit_width) = struct_declarator.bit_width {
        visitor.visit_expression(&bit_width.node, &bit_width.span);
    }
}

pub fn visit_enum_type<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    enum_type: &'ast EnumType,
    _span: &'ast Span,
) {
    if let Some(ref identifier) = enum_type.identifier {
        visitor.visit_identifier(&identifier.node, &identifier.span);
    }
    for enumerator in &enum_type.enumerators {
        visitor.visit_enumerator(&enumerator.node, &enumerator.span);
    }
}

pub fn visit_enumerator<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    enumerator: &'ast Enumerator,
    _span: &'ast Span,
) {
    visitor.visit_identifier(&enumerator.identifier.node, &enumerator.identifier.span);
    if let Some(ref expression) = enumerator.expression {
        visitor.visit_expression(&expression.node, &expression.span);
    }
    for extension in &enumerator.extensions {
        visitor.visit_extension(&extension.node, &extension.span);
    }
}

pub fn visit_type_qualifier<'ast, V: Visit<'ast> + ?Sized>(
    _visitor: &mut V,
    _type_qualifier: &'ast TypeQualifier,
    _span: &'ast Span,
) {
}

pub fn visit_function_specifier<'ast, V: Visit<'ast> + ?Sized>(
    _visitor: &mut V,
    _function_specifier: &'ast FunctionSpecifier,
    _span: &'ast Span,
) {
}

pub fn visit_alignment_specifier<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    alignment_specifier: &'ast AlignmentSpecifier,
    _span: &'ast Span,
) {
    match *alignment_specifier {
        AlignmentSpecifier::Type(ref t) => visitor.visit_type_name(&t.node, &t.span),
        AlignmentSpecifier::Constant(ref c) => visitor.visit_expression(&c.node, &c.span),
    }
}

pub fn visit_declarator<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    declarator: &'ast Declarator,
    _span: &'ast Span,
) {
    visitor.visit_declarator_kind(&declarator.kind.node, &declarator.kind.span);
    for derived in &declarator.derived {
        visitor.visit_derived_declarator(&derived.node, &derived.span);
    }
    for extension in &declarator.extensions {
        visitor.visit_extension(&extension.node, &extension.span);
    }
}

pub fn visit_declarator_kind<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    declarator_kind: &'ast DeclaratorKind,
    _span: &'ast Span,
) {
    match *declarator_kind {
        DeclaratorKind::Identifier(ref i) => visitor.visit_identifier(&i.node, &i.span),
        DeclaratorKind::Declarator(ref d) => visitor.visit_declarator(&d.node, &d.span),
        _ => {}
    }
}

pub fn visit_derived_declarator<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    derived_declarator: &'ast DerivedDeclarator,
    _span: &'ast Span,
) {
    match *derived_declarator {
        DerivedDeclarator::Pointer(ref p) => {
            for pointer in p {
                visitor.visit_pointer_qualifier(&pointer.node, &pointer.span);
            }
        }
        DerivedDeclarator::Array(ref a) => visitor.visit_array_declarator(&a.node, &a.span),
        DerivedDeclarator::Function(ref f) => visitor.visit_function_declarator(&f.node, &f.span),
        DerivedDeclarator::KRFunction(ref k) => {
            for identifier in k {
                visitor.visit_identifier(&identifier.node, &identifier.span);
            }
        }
        DerivedDeclarator::Block(ref qs) => {
            for q in qs {
                visitor.visit_pointer_qualifier(&q.node, &q.span);
            }
        }
    }
}

pub fn visit_array_declarator<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    array_declarator: &'ast ArrayDeclarator,
    span: &'ast Span,
) {
    for qualifier in &array_declarator.qualifiers {
        visitor.visit_type_qualifier(&qualifier.node, &qualifier.span);
    }
    visitor.visit_array_size(&array_declarator.size, span)
}

pub fn visit_function_declarator<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    function_declarator: &'ast FunctionDeclarator,
    span: &'ast Span,
) {
    for parameter in &function_declarator.parameters {
        visitor.visit_parameter_declaration(&parameter.node, &parameter.span);
    }
    visitor.visit_ellipsis(&function_declarator.ellipsis, span);
}

pub fn visit_pointer_qualifier<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    pointer_qualifier: &'ast PointerQualifier,
    _span: &'ast Span,
) {
    match *pointer_qualifier {
        PointerQualifier::TypeQualifier(ref t) => visitor.visit_type_qualifier(&t.node, &t.span),
        PointerQualifier::Extension(ref e) => {
            for extension in e {
                visitor.visit_extension(&extension.node, &extension.span);
            }
        }
    }
}

pub fn visit_array_size<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    array_size: &'ast ArraySize,
    _span: &'ast Span,
) {
    match *array_size {
        ArraySize::VariableExpression(ref e) => visitor.visit_expression(&e.node, &e.span),
        ArraySize::StaticExpression(ref s) => visitor.visit_expression(&s.node, &s.span),
        _ => {}
    }
}

pub fn visit_parameter_declaration<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    parameter_declaration: &'ast ParameterDeclaration,
    _span: &'ast Span,
) {
    for specifier in &parameter_declaration.specifiers {
        visitor.visit_declaration_specifier(&specifier.node, &specifier.span);
    }
    if let Some(ref declarator) = parameter_declaration.declarator {
        visitor.visit_declarator(&declarator.node, &declarator.span);
    }
    for extension in &parameter_declaration.extensions {
        visitor.visit_extension(&extension.node, &extension.span);
    }
}

pub fn visit_ellipsis<'ast, V: Visit<'ast> + ?Sized>(
    _visitor: &mut V,
    _ellipsis: &'ast Ellipsis,
    _span: &'ast Span,
) {
}

pub fn visit_type_name<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    type_name: &'ast TypeName,
    _span: &'ast Span,
) {
    for specifier in &type_name.specifiers {
        visitor.visit_specifier_qualifier(&specifier.node, &specifier.span);
    }
    if let Some(ref declarator) = type_name.declarator {
        visitor.visit_declarator(&declarator.node, &declarator.span);
    }
}

pub fn visit_initializer<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    initializer: &'ast Initializer,
    _span: &'ast Span,
) {
    match *initializer {
        Initializer::Expression(ref e) => visitor.visit_expression(&e.node, &e.span),
        Initializer::List(ref l) => {
            for item in l {
                visitor.visit_initializer_list_item(&item.node, &item.span);
            }
        }
    }
}

pub fn visit_initializer_list_item<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    initializer_list_item: &'ast InitializerListItem,
    _span: &'ast Span,
) {
    for designation in &initializer_list_item.designation {
        visitor.visit_designator(&designation.node, &designation.span);
    }
    visitor.visit_initializer(&initializer_list_item.initializer.node, &initializer_list_item.initializer.span);
}

pub fn visit_designator<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    designator: &'ast Designator,
    _span: &'ast Span,
) {
    match *designator {
        Designator::Index(ref i) => visitor.visit_expression(&i.node, &i.span),
        Designator::Member(ref m) => visitor.visit_identifier(&m.node, &m.span),
        Designator::Range(ref r) => visitor.visit_range_designator(&r.node, &r.span),
    }
}

pub fn visit_range_designator<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    range_designator: &'ast RangeDesignator,
    _span: &'ast Span,
) {
    visitor.visit_expression(&range_designator.from.node, &range_designator.from.span);
    visitor.visit_expression(&range_designator.to.node, &range_designator.to.span);
}

pub fn visit_static_assert<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    static_assert: &'ast StaticAssert,
    _span: &'ast Span,
) {
    visitor.visit_expression(&static_assert.expression.node, &static_assert.expression.span);
    visitor.visit_string_literal(&static_assert.message.node, &static_assert.message.span);
}

pub fn visit_translation_unit<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    translation_unit: &'ast TranslationUnit,
) {
    for element in &translation_unit.0 {
        visitor.visit_external_declaration(&element.node, &element.span);
    }
}

pub fn visit_external_declaration<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    external_declaration: &'ast ExternalDeclaration,
    _span: &'ast Span,
) {
    match *external_declaration {
        ExternalDeclaration::Declaration(ref d) => visitor.visit_declaration(&d.node, &d.span),
        ExternalDeclaration::StaticAssert(ref s) => visitor.visit_static_assert(&s.node, &s.span),
        ExternalDeclaration::FunctionDefinition(ref f) => visitor.visit_function_definition(&f.node, &f.span),
    }
}

pub fn visit_function_definition<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    function_definition: &'ast FunctionDefinition,
    _span: &'ast Span,
) {
    for specifier in &function_definition.specifiers {
        visitor.visit_declaration_specifier(&specifier.node, &specifier.span);
    }
    visitor.visit_declarator(&function_definition.declarator.node, &function_definition.declarator.span);
    for declaration in &function_definition.declarations {
        visitor.visit_declaration(&declaration.node, &declaration.span);
    }
    visitor.visit_statement(&function_definition.statement.node, &function_definition.statement.span);
}
