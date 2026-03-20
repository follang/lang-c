static_assert(1, "ok");

/*===
TranslationUnit
    ExternalDeclaration
        StaticAssert
            Expression
                Constant
                    Integer "1"
                        IntegerBase Decimal
                        IntegerSuffix false false
                            IntegerSize Int
            StringLiteral ["\"ok\""]
===*/
