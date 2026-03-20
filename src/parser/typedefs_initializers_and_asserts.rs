fn __parse_typedef_name<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Node<Identifier>> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            __state.suppress_fail += 1;
            let res = __parse_typedef_name0(__input, __state, __pos, env);
            __state.suppress_fail -= 1;
            res
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                __state.mark_failure(__pos, "<typedef_name>");
                Failed
            }
        }
    }
}

fn __parse_typedef_name0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Node<Identifier>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = __parse_identifier(__input, __state, __pos, env);
        match __seq_res {
            Matched(__pos, i) => {
                match {
                    if env.is_typename(&i.node.name) {
                        Ok(i)
                    } else {
                        Err("<unused>")
                    }
                } {
                    Ok(res) => Matched(__pos, res),
                    Err(expected) => {
                        __state.mark_failure(__pos, expected);
                        Failed
                    }
                }
            }
            Failed => Failed,
        }
    }
}

fn __parse_initializer<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Initializer> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = __parse_assignment_expression(__input, __state, __pos, env);
            match __seq_res {
                Matched(__pos, e) => Matched(__pos, { Initializer::Expression(e) }),
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __choice_res = {
                    let __seq_res = slice_eq(__input, __state, __pos, "{");
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = __parse__(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = {
                                        let __seq_res = {
                                            let mut __repeat_pos = __pos;
                                            let mut __repeat_value = vec![];
                                            loop {
                                                let __pos = __repeat_pos;
                                                let __pos = if __repeat_value.len() > 0 {
                                                    let __sep_res = {
                                                        let __seq_res = __parse__(__input, __state, __pos, env);
                                                        match __seq_res {
                                                            Matched(__pos, _) => {
                                                                let __seq_res = slice_eq(__input, __state, __pos, ",");
                                                                match __seq_res {
                                                                    Matched(__pos, _) => __parse__(__input, __state, __pos, env),
                                                                    Failed => Failed,
                                                                }
                                                            }
                                                            Failed => Failed,
                                                        }
                                                    };
                                                    match __sep_res {
                                                        Matched(__newpos, _) => __newpos,
                                                        Failed => break,
                                                    }
                                                } else {
                                                    __pos
                                                };
                                                let __step_res = {
                                                    let __seq_res = Matched(__pos, __pos);
                                                    match __seq_res {
                                                        Matched(__pos, l) => {
                                                            let __seq_res = __parse_initializer_list_item(__input, __state, __pos, env);
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
                                                match __step_res {
                                                    Matched(__newpos, __value) => {
                                                        __repeat_pos = __newpos;
                                                        __repeat_value.push(__value);
                                                    }
                                                    Failed => {
                                                        break;
                                                    }
                                                }
                                            }
                                            if __repeat_value.len() >= 1 {
                                                Matched(__repeat_pos, __repeat_value)
                                            } else {
                                                Failed
                                            }
                                        };
                                        match __seq_res {
                                            Matched(__pos, e) => Matched(__pos, { e }),
                                            Failed => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, i) => {
                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, _) => {
                                                    let __seq_res = match slice_eq(__input, __state, __pos, ",") {
                                                        Matched(__newpos, _) => Matched(__newpos, ()),
                                                        Failed => Matched(__pos, ()),
                                                    };
                                                    match __seq_res {
                                                        Matched(__pos, _) => {
                                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                                            match __seq_res {
                                                                Matched(__pos, _) => {
                                                                    let __seq_res = slice_eq(__input, __state, __pos, "}");
                                                                    match __seq_res {
                                                                        Matched(__pos, _) => Matched(__pos, { Initializer::List(i) }),
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
                        let __seq_res = {
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
                                        let __seq_res = slice_eq(__input, __state, __pos, "{");
                                        match __seq_res {
                                            Matched(__pos, _) => {
                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                match __seq_res {
                                                    Matched(__pos, _) => slice_eq(__input, __state, __pos, "}"),
                                                    Failed => Failed,
                                                }
                                            }
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
                        };
                        match __seq_res {
                            Matched(__pos, _) => Matched(__pos, { Initializer::List(Vec::new()) }),
                            Failed => Failed,
                        }
                    }
                }
            }
        }
    }
}

fn __parse_initializer_list_item<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<InitializerListItem> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = match __parse_designation(__input, __state, __pos, env) {
            Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
            Failed => Matched(__pos, None),
        };
        match __seq_res {
            Matched(__pos, d) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = {
                            let __seq_res = Matched(__pos, __pos);
                            match __seq_res {
                                Matched(__pos, l) => {
                                    let __seq_res = __parse_initializer(__input, __state, __pos, env);
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
                            Matched(__pos, i) => Matched(__pos, { InitializerListItem { designation: d.unwrap_or_default(), initializer: Box::new(i) } }),
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            }
            Failed => Failed,
        }
    }
}

fn __parse_designation<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Vec<Node<Designator>>> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = {
                let __seq_res = {
                    let mut __repeat_pos = __pos;
                    let mut __repeat_value = vec![];
                    loop {
                        let __pos = __repeat_pos;
                        let __pos = if __repeat_value.len() > 0 {
                            let __sep_res = __parse__(__input, __state, __pos, env);
                            match __sep_res {
                                Matched(__newpos, _) => __newpos,
                                Failed => break,
                            }
                        } else {
                            __pos
                        };
                        let __step_res = {
                            let __seq_res = Matched(__pos, __pos);
                            match __seq_res {
                                Matched(__pos, l) => {
                                    let __seq_res = __parse_designator(__input, __state, __pos, env);
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
                        match __step_res {
                            Matched(__newpos, __value) => {
                                __repeat_pos = __newpos;
                                __repeat_value.push(__value);
                            }
                            Failed => {
                                break;
                            }
                        }
                    }
                    if __repeat_value.len() >= 1 {
                        Matched(__repeat_pos, __repeat_value)
                    } else {
                        Failed
                    }
                };
                match __seq_res {
                    Matched(__pos, e) => Matched(__pos, { e }),
                    Failed => Failed,
                }
            };
            match __seq_res {
                Matched(__pos, d) => {
                    let __seq_res = __parse__(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = slice_eq(__input, __state, __pos, "=");
                            match __seq_res {
                                Matched(__pos, _) => Matched(__pos, { d }),
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
                                    let __seq_res = Matched(__pos, __pos);
                                    match __seq_res {
                                        Matched(__pos, l) => {
                                            let __seq_res = __parse_colon_designation(__input, __state, __pos, env);
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
                                    Matched(__pos, e) => Matched(__pos, { e }),
                                    Failed => Failed,
                                }
                            }
                            Failed => Failed,
                        }
                    };
                    match __seq_res {
                        Matched(__pos, d) => Matched(__pos, { vec![d] }),
                        Failed => Failed,
                    }
                };
                match __choice_res {
                    Matched(__pos, __value) => Matched(__pos, __value),
                    Failed => {
                        let __seq_res = {
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
                                        let __seq_res = Matched(__pos, __pos);
                                        match __seq_res {
                                            Matched(__pos, l) => {
                                                let __seq_res = __parse_array_designator(__input, __state, __pos, env);
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
                                        Matched(__pos, e) => Matched(__pos, { e }),
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        };
                        match __seq_res {
                            Matched(__pos, d) => Matched(__pos, { vec![d] }),
                            Failed => Failed,
                        }
                    }
                }
            }
        }
    }
}

fn __parse_colon_designation<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Designator> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = __parse_identifier(__input, __state, __pos, env);
        match __seq_res {
            Matched(__pos, i) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = slice_eq(__input, __state, __pos, ":");
                        match __seq_res {
                            Matched(__pos, _) => Matched(__pos, { Designator::Member(i) }),
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            }
            Failed => Failed,
        }
    }
}

fn __parse_designator<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Designator> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = __parse_array_designator(__input, __state, __pos, env);
            match __seq_res {
                Matched(__pos, d) => Matched(__pos, { d }),
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __seq_res = slice_eq(__input, __state, __pos, ".");
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = __parse__(__input, __state, __pos, env);
                        match __seq_res {
                            Matched(__pos, _) => {
                                let __seq_res = __parse_identifier(__input, __state, __pos, env);
                                match __seq_res {
                                    Matched(__pos, i) => Matched(__pos, { Designator::Member(i) }),
                                    Failed => Failed,
                                }
                            }
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            }
        }
    }
}

fn __parse_array_designator<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Designator> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = slice_eq(__input, __state, __pos, "[");
        match __seq_res {
            Matched(__pos, _) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = {
                            let __seq_res = Matched(__pos, __pos);
                            match __seq_res {
                                Matched(__pos, l) => {
                                    let __seq_res = __parse_constant_expression0(__input, __state, __pos, env);
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
                            Matched(__pos, a) => {
                                let __seq_res = __parse__(__input, __state, __pos, env);
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        let __seq_res = match {
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
                                                    let __seq_res = __parse_range_suffix(__input, __state, __pos, env);
                                                    match __seq_res {
                                                        Matched(__pos, e) => Matched(__pos, { e }),
                                                        Failed => Failed,
                                                    }
                                                }
                                                Failed => Failed,
                                            }
                                        } {
                                            Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                                            Failed => Matched(__pos, None),
                                        };
                                        match __seq_res {
                                            Matched(__pos, b) => {
                                                let __seq_res = slice_eq(__input, __state, __pos, "]");
                                                match __seq_res {
                                                    Matched(__pos, _) => Matched(__pos, {
                                                        match b {
                                                            Some(b) => {
                                                                let span = Span::span(a.span.start, b.span.end);
                                                                Designator::Range(Node::new(RangeDesignator { from: a, to: b }, span))
                                                            }
                                                            None => Designator::Index(a),
                                                        }
                                                    }),
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
}

fn __parse_range_suffix<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Node<Expression>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = slice_eq(__input, __state, __pos, "...");
        match __seq_res {
            Matched(__pos, _) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = {
                            let __seq_res = Matched(__pos, __pos);
                            match __seq_res {
                                Matched(__pos, l) => {
                                    let __seq_res = __parse_constant_expression0(__input, __state, __pos, env);
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
                            Matched(__pos, e) => Matched(__pos, { e }),
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            }
            Failed => Failed,
        }
    }
}

fn __parse_static_assert<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Node<StaticAssert>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = Matched(__pos, __pos);
        match __seq_res {
            Matched(__pos, l) => {
                let __seq_res = __parse_static_assert0(__input, __state, __pos, env);
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
    }
}

fn __parse_static_assert0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<StaticAssert> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = match {
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
                            let __seq_res = slice_eq(__input, __state, __pos, "__extension__");
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
                        Matched(__pos, e) => Matched(__pos, { e }),
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        } {
            Matched(__newpos, _) => Matched(__newpos, ()),
            Failed => Matched(__pos, ()),
        };
        match __seq_res {
            Matched(__pos, _) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = {
                            __state.suppress_fail += 1;
                            let res = {
                                let __seq_res = {
                                    let __choice_res = slice_eq(__input, __state, __pos, "_Static_assert");
                                    match __choice_res {
                                        Matched(__pos, __value) => Matched(__pos, __value),
                                        Failed => slice_eq(__input, __state, __pos, "static_assert"),
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
                                                            Matched(__pos, e) => {
                                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                                match __seq_res {
                                                                    Matched(__pos, _) => {
                                                                        let __seq_res = slice_eq(__input, __state, __pos, ",");
                                                                        match __seq_res {
                                                                            Matched(__pos, _) => {
                                                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                                                match __seq_res {
                                                                                    Matched(__pos, _) => {
                                                                                        let __seq_res = __parse_string_literal(__input, __state, __pos, env);
                                                                                        match __seq_res {
                                                                                            Matched(__pos, s) => {
                                                                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                                                                match __seq_res {
                                                                                                    Matched(__pos, _) => {
                                                                                                        let __seq_res = slice_eq(__input, __state, __pos, ")");
                                                                                                        match __seq_res {
                                                                                                            Matched(__pos, _) => {
                                                                                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                                                                                match __seq_res {
                                                                                                                    Matched(__pos, _) => {
                                                                                                                        let __seq_res = slice_eq(__input, __state, __pos, ";");
                                                                                                                        match __seq_res {
                                                                                                                            Matched(__pos, _) => Matched(__pos, { StaticAssert { expression: e, message: s } }),
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
                    }
                    Failed => Failed,
                }
            }
            Failed => Failed,
        }
    }
}

