fn __parse_type_specifier_unique<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<TypeSpecifier> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = {
                __state.suppress_fail += 1;
                let res = {
                    let __seq_res = slice_eq(__input, __state, __pos, "void");
                    match __seq_res {
                        Matched(__pos, e) => {
                            let __seq_res = {
                                __state.suppress_fail += 1;
                                let __assert_res = if __input.len() > __pos {
                                    let (__ch, __next) = char_range_at(__input, __pos);
                                    match __ch {
                                        '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                        _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                    }
                                } else {
                                    __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                                };
                                __state.suppress_fail -= 1;
                                match __assert_res {
                                    Failed => Matched(__pos, ()),
                                    Matched(..) => Failed,
                                }
                            };
                            match __seq_res {
                                Matched(__pos, _) => Matched(__pos, { e }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                };
                __state.suppress_fail -= 1;
                res
            };
            match __seq_res {
                Matched(__pos, _) => Matched(__pos, { TypeSpecifier::Void }),
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __choice_res = {
                    let __seq_res = {
                        __state.suppress_fail += 1;
                        let res = {
                            let __seq_res = {
                                let __choice_res = slice_eq(__input, __state, __pos, "_Bool");
                                match __choice_res {
                                    Matched(__pos, __value) => Matched(__pos, __value),
                                    Failed => slice_eq(__input, __state, __pos, "bool"),
                                }
                            };
                            match __seq_res {
                                Matched(__pos, e) => {
                                    let __seq_res = {
                                        __state.suppress_fail += 1;
                                        let __assert_res = if __input.len() > __pos {
                                            let (__ch, __next) = char_range_at(__input, __pos);
                                            match __ch {
                                                '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                                _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                            }
                                        } else {
                                            __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                                        };
                                        __state.suppress_fail -= 1;
                                        match __assert_res {
                                            Failed => Matched(__pos, ()),
                                            Matched(..) => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, _) => Matched(__pos, { e }),
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        };
                        __state.suppress_fail -= 1;
                        res
                    };
                    match __seq_res {
                        Matched(__pos, _) => Matched(__pos, { TypeSpecifier::Bool }),
                        Failed => Failed,
                    }
                };
                match __choice_res {
                    Matched(__pos, __value) => Matched(__pos, __value),
                    Failed => {
                        let __choice_res = {
                            let __seq_res = {
                                __state.suppress_fail += 1;
                                let __assert_res = __parse_gnu_guard(__input, __state, __pos, env);
                                __state.suppress_fail -= 1;
                                match __assert_res {
                                    Matched(_, __value) => Matched(__pos, __value),
                                    Failed => Failed,
                                }
                            };
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = {
                                        __state.suppress_fail += 1;
                                        let res = {
                                            let __seq_res = slice_eq(__input, __state, __pos, "__float128");
                                            match __seq_res {
                                                Matched(__pos, e) => {
                                                    let __seq_res = {
                                                        __state.suppress_fail += 1;
                                                        let __assert_res = if __input.len() > __pos {
                                                            let (__ch, __next) = char_range_at(__input, __pos);
                                                            match __ch {
                                                                '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                                                _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                                            }
                                                        } else {
                                                            __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                                                        };
                                                        __state.suppress_fail -= 1;
                                                        match __assert_res {
                                                            Failed => Matched(__pos, ()),
                                                            Matched(..) => Failed,
                                                        }
                                                    };
                                                    match __seq_res {
                                                        Matched(__pos, _) => Matched(__pos, { e }),
                                                        Failed => Failed,
                                                    }
                                                }
                                                Failed => Failed,
                                            }
                                        };
                                        __state.suppress_fail -= 1;
                                        res
                                    };
                                    match __seq_res {
                                        Matched(__pos, _) => Matched(__pos, { TypeSpecifier::Float128 }),
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        };
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => {
                        let __choice_res = {
                            let __seq_res = {
                                __state.suppress_fail += 1;
                                let res = {
                                    let __seq_res = slice_eq(__input, __state, __pos, "_BitInt");
                                    match __seq_res {
                                        Matched(__pos, e) => {
                                            let __seq_res = {
                                                __state.suppress_fail += 1;
                                                let __assert_res = if __input.len() > __pos {
                                                    let (__ch, __next) = char_range_at(__input, __pos);
                                                    match __ch {
                                                        '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                                        _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                                    }
                                                } else {
                                                    __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                                                };
                                                __state.suppress_fail -= 1;
                                                match __assert_res {
                                                    Failed => Matched(__pos, ()),
                                                    Matched(..) => Failed,
                                                }
                                            };
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { e }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                };
                                __state.suppress_fail -= 1;
                                res
                            };
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = __parse__(__input, __state, __pos, env);
                                    match __seq_res {
                                        Matched(__pos, _) => {
                                            let __seq_res = slice_eq(__input, __state, __pos, "(");
                                            match __seq_res {
                                                Matched(__pos, _) => {
                                                    let __seq_res = __parse__(__input, __state, __pos, env);
                                                    match __seq_res {
                                                        Matched(__pos, _) => {
                                                            let __seq_res = __parse_constant_expression(__input, __state, __pos, env);
                                                            match __seq_res {
                                                                Matched(__pos, w) => {
                                                                    let __seq_res = __parse__(__input, __state, __pos, env);
                                                                    match __seq_res {
                                                                        Matched(__pos, _) => {
                                                                            let __seq_res = slice_eq(__input, __state, __pos, ")");
                                                                            match __seq_res {
                                                                                Matched(__pos, _) => Matched(__pos, { TypeSpecifier::BitInt(w) }),
                                                                                Failed => Failed,
                                                                            }
                                                                        }
                                                                        Failed => Failed,
                                                                    }
                                                                }
                                                                Failed => Failed,
                                                            }
                                                        }
                                                        Failed => Failed,
                                                    }
                                                }
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        };
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => {
                        let __choice_res = {
                            let __seq_res = {
                                __state.suppress_fail += 1;
                                let res = {
                                    let __seq_res = slice_eq(__input, __state, __pos, "_Atomic");
                                    match __seq_res {
                                        Matched(__pos, e) => {
                                            let __seq_res = {
                                                __state.suppress_fail += 1;
                                                let __assert_res = if __input.len() > __pos {
                                                    let (__ch, __next) = char_range_at(__input, __pos);
                                                    match __ch {
                                                        '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                                        _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                                    }
                                                } else {
                                                    __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                                                };
                                                __state.suppress_fail -= 1;
                                                match __assert_res {
                                                    Failed => Matched(__pos, ()),
                                                    Matched(..) => Failed,
                                                }
                                            };
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { e }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                };
                                __state.suppress_fail -= 1;
                                res
                            };
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = __parse__(__input, __state, __pos, env);
                                    match __seq_res {
                                        Matched(__pos, _) => {
                                            let __seq_res = slice_eq(__input, __state, __pos, "(");
                                            match __seq_res {
                                                Matched(__pos, _) => {
                                                    let __seq_res = __parse__(__input, __state, __pos, env);
                                                    match __seq_res {
                                                        Matched(__pos, _) => {
                                                            let __seq_res = __parse_type_name(__input, __state, __pos, env);
                                                            match __seq_res {
                                                                Matched(__pos, t) => {
                                                                    let __seq_res = __parse__(__input, __state, __pos, env);
                                                                    match __seq_res {
                                                                        Matched(__pos, _) => {
                                                                            let __seq_res = slice_eq(__input, __state, __pos, ")");
                                                                            match __seq_res {
                                                                                Matched(__pos, _) => Matched(__pos, { TypeSpecifier::Atomic(t) }),
                                                                                Failed => Failed,
                                                                            }
                                                                        }
                                                                        Failed => Failed,
                                                                    }
                                                                }
                                                                Failed => Failed,
                                                            }
                                                        }
                                                        Failed => Failed,
                                                    }
                                                }
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        };
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => {
                                let __choice_res = {
                                    let __seq_res = {
                                        let __seq_res = Matched(__pos, __pos);
                                        match __seq_res {
                                            Matched(__pos, l) => {
                                                let __seq_res = __parse_struct_or_union_specifier(__input, __state, __pos, env);
                                                match __seq_res {
                                                    Matched(__pos, e) => {
                                                        let __seq_res = Matched(__pos, __pos);
                                                        match __seq_res {
                                                            Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, s) => Matched(__pos, { TypeSpecifier::Struct(s) }),
                                        Failed => Failed,
                                    }
                                };
                                match __choice_res {
                                    Matched(__pos, __value) => Matched(__pos, __value),
                                    Failed => {
                                        let __choice_res = {
                                            let __seq_res = {
                                                let __seq_res = Matched(__pos, __pos);
                                                match __seq_res {
                                                    Matched(__pos, l) => {
                                                        let __seq_res = __parse_enum_specifier(__input, __state, __pos, env);
                                                        match __seq_res {
                                                            Matched(__pos, e) => {
                                                                let __seq_res = Matched(__pos, __pos);
                                                                match __seq_res {
                                                                    Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                                                    Failed => Failed,
                                                                }
                                                            }
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            };
                                            match __seq_res {
                                                Matched(__pos, e) => Matched(__pos, { TypeSpecifier::Enum(e) }),
                                                Failed => Failed,
                                            }
                                        };
                                        match __choice_res {
                                            Matched(__pos, __value) => Matched(__pos, __value),
                                            Failed => {
                                                let __seq_res = __parse_typedef_name(__input, __state, __pos, env);
                                                match __seq_res {
                                                    Matched(__pos, t) => Matched(__pos, { TypeSpecifier::TypedefName(t) }),
                                                    Failed => Failed,
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
                }
            }
                }
            }
        }
    }
}

fn __parse_type_specifier_nonunique<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<TypeSpecifier> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = {
                __state.suppress_fail += 1;
                let res = {
                    let __seq_res = slice_eq(__input, __state, __pos, "char");
                    match __seq_res {
                        Matched(__pos, e) => {
                            let __seq_res = {
                                __state.suppress_fail += 1;
                                let __assert_res = if __input.len() > __pos {
                                    let (__ch, __next) = char_range_at(__input, __pos);
                                    match __ch {
                                        '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                        _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                    }
                                } else {
                                    __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                                };
                                __state.suppress_fail -= 1;
                                match __assert_res {
                                    Failed => Matched(__pos, ()),
                                    Matched(..) => Failed,
                                }
                            };
                            match __seq_res {
                                Matched(__pos, _) => Matched(__pos, { e }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                };
                __state.suppress_fail -= 1;
                res
            };
            match __seq_res {
                Matched(__pos, _) => Matched(__pos, { TypeSpecifier::Char }),
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __choice_res = {
                    let __seq_res = {
                        __state.suppress_fail += 1;
                        let res = {
                            let __seq_res = slice_eq(__input, __state, __pos, "short");
                            match __seq_res {
                                Matched(__pos, e) => {
                                    let __seq_res = {
                                        __state.suppress_fail += 1;
                                        let __assert_res = if __input.len() > __pos {
                                            let (__ch, __next) = char_range_at(__input, __pos);
                                            match __ch {
                                                '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                                _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                            }
                                        } else {
                                            __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                                        };
                                        __state.suppress_fail -= 1;
                                        match __assert_res {
                                            Failed => Matched(__pos, ()),
                                            Matched(..) => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, _) => Matched(__pos, { e }),
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        };
                        __state.suppress_fail -= 1;
                        res
                    };
                    match __seq_res {
                        Matched(__pos, _) => Matched(__pos, { TypeSpecifier::Short }),
                        Failed => Failed,
                    }
                };
                match __choice_res {
                    Matched(__pos, __value) => Matched(__pos, __value),
                    Failed => {
                        let __choice_res = {
                            let __seq_res = {
                                __state.suppress_fail += 1;
                                let res = {
                                    let __seq_res = slice_eq(__input, __state, __pos, "int");
                                    match __seq_res {
                                        Matched(__pos, e) => {
                                            let __seq_res = {
                                                __state.suppress_fail += 1;
                                                let __assert_res = if __input.len() > __pos {
                                                    let (__ch, __next) = char_range_at(__input, __pos);
                                                    match __ch {
                                                        '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                                        _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                                    }
                                                } else {
                                                    __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                                                };
                                                __state.suppress_fail -= 1;
                                                match __assert_res {
                                                    Failed => Matched(__pos, ()),
                                                    Matched(..) => Failed,
                                                }
                                            };
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { e }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                };
                                __state.suppress_fail -= 1;
                                res
                            };
                            match __seq_res {
                                Matched(__pos, _) => Matched(__pos, { TypeSpecifier::Int }),
                                Failed => Failed,
                            }
                        };
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => {
                                let __choice_res = {
                                    let __seq_res = {
                                        __state.suppress_fail += 1;
                                        let res = {
                                            let __seq_res = slice_eq(__input, __state, __pos, "long");
                                            match __seq_res {
                                                Matched(__pos, e) => {
                                                    let __seq_res = {
                                                        __state.suppress_fail += 1;
                                                        let __assert_res = if __input.len() > __pos {
                                                            let (__ch, __next) = char_range_at(__input, __pos);
                                                            match __ch {
                                                                '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                                                _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                                            }
                                                        } else {
                                                            __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                                                        };
                                                        __state.suppress_fail -= 1;
                                                        match __assert_res {
                                                            Failed => Matched(__pos, ()),
                                                            Matched(..) => Failed,
                                                        }
                                                    };
                                                    match __seq_res {
                                                        Matched(__pos, _) => Matched(__pos, { e }),
                                                        Failed => Failed,
                                                    }
                                                }
                                                Failed => Failed,
                                            }
                                        };
                                        __state.suppress_fail -= 1;
                                        res
                                    };
                                    match __seq_res {
                                        Matched(__pos, _) => Matched(__pos, { TypeSpecifier::Long }),
                                        Failed => Failed,
                                    }
                                };
                                match __choice_res {
                                    Matched(__pos, __value) => Matched(__pos, __value),
                                    Failed => {
                                        let __choice_res = {
                                            let __seq_res = {
                                                __state.suppress_fail += 1;
                                                let res = {
                                                    let __seq_res = slice_eq(__input, __state, __pos, "float");
                                                    match __seq_res {
                                                        Matched(__pos, e) => {
                                                            let __seq_res = {
                                                                __state.suppress_fail += 1;
                                                                let __assert_res = if __input.len() > __pos {
                                                                    let (__ch, __next) = char_range_at(__input, __pos);
                                                                    match __ch {
                                                                        '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                                                        _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                                                    }
                                                                } else {
                                                                    __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                                                                };
                                                                __state.suppress_fail -= 1;
                                                                match __assert_res {
                                                                    Failed => Matched(__pos, ()),
                                                                    Matched(..) => Failed,
                                                                }
                                                            };
                                                            match __seq_res {
                                                                Matched(__pos, _) => Matched(__pos, { e }),
                                                                Failed => Failed,
                                                            }
                                                        }
                                                        Failed => Failed,
                                                    }
                                                };
                                                __state.suppress_fail -= 1;
                                                res
                                            };
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { TypeSpecifier::Float }),
                                                Failed => Failed,
                                            }
                                        };
                                        match __choice_res {
                                            Matched(__pos, __value) => Matched(__pos, __value),
                                            Failed => {
                                                let __choice_res = {
                                                    let __seq_res = {
                                                        __state.suppress_fail += 1;
                                                        let res = {
                                                            let __seq_res = slice_eq(__input, __state, __pos, "double");
                                                            match __seq_res {
                                                                Matched(__pos, e) => {
                                                                    let __seq_res = {
                                                                        __state.suppress_fail += 1;
                                                                        let __assert_res = if __input.len() > __pos {
                                                                            let (__ch, __next) = char_range_at(__input, __pos);
                                                                            match __ch {
                                                                                '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                                                                _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                                                            }
                                                                        } else {
                                                                            __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                                                                        };
                                                                        __state.suppress_fail -= 1;
                                                                        match __assert_res {
                                                                            Failed => Matched(__pos, ()),
                                                                            Matched(..) => Failed,
                                                                        }
                                                                    };
                                                                    match __seq_res {
                                                                        Matched(__pos, _) => Matched(__pos, { e }),
                                                                        Failed => Failed,
                                                                    }
                                                                }
                                                                Failed => Failed,
                                                            }
                                                        };
                                                        __state.suppress_fail -= 1;
                                                        res
                                                    };
                                                    match __seq_res {
                                                        Matched(__pos, _) => Matched(__pos, { TypeSpecifier::Double }),
                                                        Failed => Failed,
                                                    }
                                                };
                                                match __choice_res {
                                                    Matched(__pos, __value) => Matched(__pos, __value),
                                                    Failed => {
                                                        let __choice_res = {
                                                            let __seq_res = {
                                                                __state.suppress_fail += 1;
                                                                let res = {
                                                                    let __seq_res = {
                                                                        let __choice_res = slice_eq(__input, __state, __pos, "signed");
                                                                        match __choice_res {
                                                                            Matched(__pos, __value) => Matched(__pos, __value),
                                                                            Failed => {
                                                                                let __seq_res = {
                                                                                    __state.suppress_fail += 1;
                                                                                    let __assert_res = __parse_gnu_guard(__input, __state, __pos, env);
                                                                                    __state.suppress_fail -= 1;
                                                                                    match __assert_res {
                                                                                        Matched(_, __value) => Matched(__pos, __value),
                                                                                        Failed => Failed,
                                                                                    }
                                                                                };
                                                                                match __seq_res {
                                                                                    Matched(__pos, _) => {
                                                                                        let __seq_res = {
                                                                                            let __seq_res = slice_eq(__input, __state, __pos, "__signed");
                                                                                            match __seq_res {
                                                                                                Matched(__pos, _) => match slice_eq(__input, __state, __pos, "__") {
                                                                                                    Matched(__newpos, _) => Matched(__newpos, ()),
                                                                                                    Failed => Matched(__pos, ()),
                                                                                                },
                                                                                                Failed => Failed,
                                                                                            }
                                                                                        };
                                                                                        match __seq_res {
                                                                                            Matched(__pos, e) => Matched(__pos, { e }),
                                                                                            Failed => Failed,
                                                                                        }
                                                                                    }
                                                                                    Failed => Failed,
                                                                                }
                                                                            }
                                                                        }
                                                                    };
                                                                    match __seq_res {
                                                                        Matched(__pos, e) => {
                                                                            let __seq_res = {
                                                                                __state.suppress_fail += 1;
                                                                                let __assert_res = if __input.len() > __pos {
                                                                                    let (__ch, __next) = char_range_at(__input, __pos);
                                                                                    match __ch {
                                                                                        '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                                                                        _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                                                                    }
                                                                                } else {
                                                                                    __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                                                                                };
                                                                                __state.suppress_fail -= 1;
                                                                                match __assert_res {
                                                                                    Failed => Matched(__pos, ()),
                                                                                    Matched(..) => Failed,
                                                                                }
                                                                            };
                                                                            match __seq_res {
                                                                                Matched(__pos, _) => Matched(__pos, { e }),
                                                                                Failed => Failed,
                                                                            }
                                                                        }
                                                                        Failed => Failed,
                                                                    }
                                                                };
                                                                __state.suppress_fail -= 1;
                                                                res
                                                            };
                                                            match __seq_res {
                                                                Matched(__pos, _) => Matched(__pos, { TypeSpecifier::Signed }),
                                                                Failed => Failed,
                                                            }
                                                        };
                                                        match __choice_res {
                                                            Matched(__pos, __value) => Matched(__pos, __value),
                                                            Failed => {
                                                                let __choice_res = {
                                                                    let __seq_res = {
                                                                        __state.suppress_fail += 1;
                                                                        let res = {
                                                                            let __seq_res = slice_eq(__input, __state, __pos, "unsigned");
                                                                            match __seq_res {
                                                                                Matched(__pos, e) => {
                                                                                    let __seq_res = {
                                                                                        __state.suppress_fail += 1;
                                                                                        let __assert_res = if __input.len() > __pos {
                                                                                            let (__ch, __next) = char_range_at(__input, __pos);
                                                                                            match __ch {
                                                                                                '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                                                                                _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                                                                            }
                                                                                        } else {
                                                                                            __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                                                                                        };
                                                                                        __state.suppress_fail -= 1;
                                                                                        match __assert_res {
                                                                                            Failed => Matched(__pos, ()),
                                                                                            Matched(..) => Failed,
                                                                                        }
                                                                                    };
                                                                                    match __seq_res {
                                                                                        Matched(__pos, _) => Matched(__pos, { e }),
                                                                                        Failed => Failed,
                                                                                    }
                                                                                }
                                                                                Failed => Failed,
                                                                            }
                                                                        };
                                                                        __state.suppress_fail -= 1;
                                                                        res
                                                                    };
                                                                    match __seq_res {
                                                                        Matched(__pos, _) => Matched(__pos, { TypeSpecifier::Unsigned }),
                                                                        Failed => Failed,
                                                                    }
                                                                };
                                                                match __choice_res {
                                                                    Matched(__pos, __value) => Matched(__pos, __value),
                                                                    Failed => {
                                                                        let __choice_res = {
                                                                            let __seq_res = {
                                                                                __state.suppress_fail += 1;
                                                                                let res = {
                                                                                    let __seq_res = {
                                                                                        let __choice_res = {
                                                                                            let __choice_res = slice_eq(__input, __state, __pos, "_Complex");
                                                                                            match __choice_res {
                                                                                                Matched(__pos, __value) => Matched(__pos, __value),
                                                                                                Failed => slice_eq(__input, __state, __pos, "complex"),
                                                                                            }
                                                                                        };
                                                                                        match __choice_res {
                                                                                            Matched(__pos, __value) => Matched(__pos, __value),
                                                                                            Failed => {
                                                                                                let __seq_res = {
                                                                                                    __state.suppress_fail += 1;
                                                                                                    let __assert_res = __parse_gnu_guard(__input, __state, __pos, env);
                                                                                                    __state.suppress_fail -= 1;
                                                                                                    match __assert_res {
                                                                                                        Matched(_, __value) => Matched(__pos, __value),
                                                                                                        Failed => Failed,
                                                                                                    }
                                                                                                };
                                                                                                match __seq_res {
                                                                                                    Matched(__pos, _) => {
                                                                                                        let __seq_res = {
                                                                                                            let __seq_res = slice_eq(__input, __state, __pos, "__complex");
                                                                                                            match __seq_res {
                                                                                                                Matched(__pos, _) => match slice_eq(__input, __state, __pos, "__") {
                                                                                                                    Matched(__newpos, _) => Matched(__newpos, ()),
                                                                                                                    Failed => Matched(__pos, ()),
                                                                                                                },
                                                                                                                Failed => Failed,
                                                                                                            }
                                                                                                        };
                                                                                                        match __seq_res {
                                                                                                            Matched(__pos, e) => Matched(__pos, { e }),
                                                                                                            Failed => Failed,
                                                                                                        }
                                                                                                    }
                                                                                                    Failed => Failed,
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    };
                                                                                    match __seq_res {
                                                                                        Matched(__pos, e) => {
                                                                                            let __seq_res = {
                                                                                                __state.suppress_fail += 1;
                                                                                                let __assert_res = if __input.len() > __pos {
                                                                                                    let (__ch, __next) = char_range_at(__input, __pos);
                                                                                                    match __ch {
                                                                                                        '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                                                                                        _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                                                                                    }
                                                                                                } else {
                                                                                                    __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                                                                                                };
                                                                                                __state.suppress_fail -= 1;
                                                                                                match __assert_res {
                                                                                                    Failed => Matched(__pos, ()),
                                                                                                    Matched(..) => Failed,
                                                                                                }
                                                                                            };
                                                                                            match __seq_res {
                                                                                                Matched(__pos, _) => Matched(__pos, { e }),
                                                                                                Failed => Failed,
                                                                                            }
                                                                                        }
                                                                                        Failed => Failed,
                                                                                    }
                                                                                };
                                                                                __state.suppress_fail -= 1;
                                                                                res
                                                                            };
                                                                            match __seq_res {
                                                                                Matched(__pos, _) => Matched(__pos, { TypeSpecifier::Complex }),
                                                                                Failed => Failed,
                                                                            }
                                                                        };
                                                                        match __choice_res {
                                                                            Matched(__pos, __value) => Matched(__pos, __value),
                                                                            Failed => {
                                                                                let __choice_res = {
                                                                                    let __seq_res = {
                                                                                        __state.suppress_fail += 1;
                                                                                        let res = {
                                                                                            let __seq_res = __parse_ts18661_float_type_specifier(__input, __state, __pos, env);
                                                                                            match __seq_res {
                                                                                                Matched(__pos, e) => {
                                                                                                    let __seq_res = {
                                                                                                        __state.suppress_fail += 1;
                                                                                                        let __assert_res = if __input.len() > __pos {
                                                                                                            let (__ch, __next) = char_range_at(__input, __pos);
                                                                                                            match __ch {
                                                                                                                '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                                                                                                _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                                                                                            }
                                                                                                        } else {
                                                                                                            __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                                                                                                        };
                                                                                                        __state.suppress_fail -= 1;
                                                                                                        match __assert_res {
                                                                                                            Failed => Matched(__pos, ()),
                                                                                                            Matched(..) => Failed,
                                                                                                        }
                                                                                                    };
                                                                                                    match __seq_res {
                                                                                                        Matched(__pos, _) => Matched(__pos, { e }),
                                                                                                        Failed => Failed,
                                                                                                    }
                                                                                                }
                                                                                                Failed => Failed,
                                                                                            }
                                                                                        };
                                                                                        __state.suppress_fail -= 1;
                                                                                        res
                                                                                    };
                                                                                    match __seq_res {
                                                                                        Matched(__pos, t) => Matched(__pos, { TypeSpecifier::TS18661Float(t) }),
                                                                                        Failed => Failed,
                                                                                    }
                                                                                };
                                                                                match __choice_res {
                                                                                    Matched(__pos, __value) => Matched(__pos, __value),
                                                                                    Failed => {
                                                                                        let __choice_res = {
                                                                                            let __seq_res = {
                                                                                                __state.suppress_fail += 1;
                                                                                                let __assert_res = __parse_gnu_guard(__input, __state, __pos, env);
                                                                                                __state.suppress_fail -= 1;
                                                                                                match __assert_res {
                                                                                                    Matched(_, __value) => Matched(__pos, __value),
                                                                                                    Failed => Failed,
                                                                                                }
                                                                                            };
                                                                                            match __seq_res {
                                                                                                Matched(__pos, _) => {
                                                                                                    let __seq_res = {
                                                                                                        __state.suppress_fail += 1;
                                                                                                        let res = {
                                                                                                            let __seq_res = slice_eq(__input, __state, __pos, "__int128");
                                                                                                            match __seq_res {
                                                                                                                Matched(__pos, e) => {
                                                                                                                    let __seq_res = {
                                                                                                                        __state.suppress_fail += 1;
                                                                                                                        let __assert_res = if __input.len() > __pos {
                                                                                                                            let (__ch, __next) = char_range_at(__input, __pos);
                                                                                                                            match __ch {
                                                                                                                                '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                                                                                                                _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                                                                                                            }
                                                                                                                        } else {
                                                                                                                            __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                                                                                                                        };
                                                                                                                        __state.suppress_fail -= 1;
                                                                                                                        match __assert_res {
                                                                                                                            Failed => Matched(__pos, ()),
                                                                                                                            Matched(..) => Failed,
                                                                                                                        }
                                                                                                                    };
                                                                                                                    match __seq_res {
                                                                                                                        Matched(__pos, _) => Matched(__pos, { e }),
                                                                                                                        Failed => Failed,
                                                                                                                    }
                                                                                                                }
                                                                                                                Failed => Failed,
                                                                                                            }
                                                                                                        };
                                                                                                        __state.suppress_fail -= 1;
                                                                                                        res
                                                                                                    };
                                                                                                    match __seq_res {
                                                                                                        Matched(__pos, _) => Matched(__pos, { TypeSpecifier::Int128 }),
                                                                                                        Failed => Failed,
                                                                                                    }
                                                                                                }
                                                                                                Failed => Failed,
                                                                                            }
                                                                                        };
                                                                                        match __choice_res {
                                                                                            Matched(__pos, __value) => Matched(__pos, __value),
                                                                                            Failed => {
                                                                                                let __seq_res = __parse_typeof_specifier(__input, __state, __pos, env);
                                                                                                match __seq_res {
                                                                                                    Matched(__pos, e) => Matched(__pos, { e }),
                                                                                                    Failed => Failed,
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

