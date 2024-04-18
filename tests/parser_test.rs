use rost_interpreter::{
    ast::{
        Boolean, CallExpression, Expression, ExpressionStatement, FunctionLiteral, Identifier,
        IfExpression, InfixExpression, IntegerLiteral, LetStatement, Node, PrefixExpression,
        ReturnStatement,
    },
    lexer::Lexer,
    parser::Parser,
};

fn test_integer_literal(integer_expression: &dyn Expression, value: i64) {
    if let Some(integer_literal) = integer_expression.as_any().downcast_ref::<IntegerLiteral>() {
        assert_eq!(integer_literal.value, value);
        assert_eq!(integer_literal.token_literal(), value.to_string());
    } else {
        panic!("Expected: IntegerLiteral\nGot: {:?}", integer_expression);
    }
}

#[test]
fn test_let_statements() {
    #[derive(Debug)]
    enum Used {
        Int(i64),
        Bool(bool),
        String(String),
    }
    #[derive(Debug)]
    struct Test {
        input: Vec<u8>,
        expected_identifier: String,
        expected_value: Used,
    }
    let tests: Vec<Test> = vec![
        Test {
            input: b"let x = 5;".to_vec(),
            expected_identifier: String::from("x"),
            expected_value: Used::Int(5),
        },
        Test {
            input: b"let y = true;".to_vec(),
            expected_identifier: String::from("y"),
            expected_value: Used::Bool(true),
        },
        Test {
            input: b"let foobar = y;".to_vec(),
            expected_identifier: String::from("foobar"),
            expected_value: Used::String(String::from("y")),
        },
    ];
    for test in tests {
        let lexer = Lexer::new(&test.input);
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);
        let statement = match program.statements[0]
            .as_any()
            .downcast_ref::<LetStatement>()
        {
            Some(l) => l,
            None => panic!(
                "Not the right kind of Statement. Expected: LetStatement\nGot: {}",
                &program.statements[0]
            ),
        };
        assert_eq!(statement.name.value, test.expected_identifier);
        assert_eq!(statement.name.token_literal(), test.expected_identifier);
        dbg!(&statement.value);
        match test.expected_value {
            Used::Int(i) => {
                let return_value = match statement.value.as_any().downcast_ref::<IntegerLiteral>() {
                    Some(i) => i,
                    None => panic!(
                        "Not the right kind of Expression. Expected: IntegerLiteral\nGot: {}",
                        statement.value
                    ),
                };
                assert_eq!(return_value.value, i)
            }
            Used::Bool(b) => {
                let return_value = match statement.value.as_any().downcast_ref::<Boolean>() {
                        Some(b) => b,
                        None => panic!(
                            "Not the right kind of Expression. Expected: Expression::Identifier\nGot: {}",
                            statement.value
                        ),
                    };
                assert_eq!(return_value.value, b)
            }
            Used::String(s) => {
                let return_value = match statement.value.as_any().downcast_ref::<Identifier>() {
                        Some(i) => i,
                        None => panic!(
                            "Not the right kind of Expression. Expected: Expression::Identifier\nGot: {}",
                            statement.value
                        ),
                    };
                assert_eq!(return_value.value, s)
            }
        }
    }
}

#[test]
fn test_return_statements() {
    #[derive(Debug)]
    enum Used {
        Int(i64),
        Bool(bool),
        String(String),
    }
    #[derive(Debug)]
    struct Test {
        input: Vec<u8>,
        expected_value: Used,
    }
    let tests: Vec<Test> = vec![
        Test {
            input: b"return 5;".to_vec(),
            expected_value: Used::Int(5),
        },
        Test {
            input: b"return true;".to_vec(),
            expected_value: Used::Bool(true),
        },
        Test {
            input: b"return foobar;".to_vec(),
            expected_value: Used::String(String::from("foobar")),
        },
    ];
    for test in tests {
        let lexer = Lexer::new(&test.input);
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);
        let return_statement = match program.statements[0]
            .as_any()
            .downcast_ref::<ReturnStatement>()
        {
            Some(r) => r,
            None => panic!(
                "Not the right kind of Statement. Expected: ReturnStatement\nGot: {}",
                program.statements[0]
            ),
        };
        match test.expected_value {
            Used::Int(i) => {
                let return_value = match return_statement
                    .return_value
                    .as_any()
                    .downcast_ref::<IntegerLiteral>()
                {
                    Some(i) => i,
                    None => panic!(
                        "Not the right kind of Expression. Expected: IntegerLiteral\nGot: {}",
                        return_statement.return_value
                    ),
                };
                assert_eq!(return_value.value, i)
            }
            Used::Bool(b) => {
                let return_value = match return_statement
                    .return_value
                    .as_any()
                    .downcast_ref::<Boolean>()
                {
                    Some(b) => b,
                    None => panic!(
                        "Not the right kind of Expression. Expected: Boolean\nGot: {}",
                        return_statement.return_value
                    ),
                };
                assert_eq!(return_value.value, b)
            }
            Used::String(s) => {
                let return_value = match return_statement
                    .return_value
                    .as_any()
                    .downcast_ref::<Identifier>()
                {
                    Some(i) => i,
                    None => panic!(
                        "Not the right kind of Expression. Expected: Identifier\nGot: {}",
                        return_statement.return_value
                    ),
                };
                assert_eq!(return_value.value, s)
            }
        }
    }
}

#[test]
fn test_identifier_expression() {
    let input = b"foobar";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();

    assert_eq!(program.statements.len(), 1);
    if let Some(expression_statement) = program.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStatement>()
    {
        if let Some(identifier) = expression_statement
            .expression
            .as_any()
            .downcast_ref::<Identifier>()
        {
            assert_eq!(identifier.value, "foobar");
            assert_eq!(identifier.token_literal(), "foobar");
        } else {
            panic!("Expected: Identifier\nGot: {:?}", expression_statement);
        }
    } else {
        panic!(
            "Expected: ExpressionStatement\nGot: {:?}",
            program.statements[0]
        );
    }
}

#[test]
fn test_integer_literal_expression() {
    let input = b"5;";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();

    assert_eq!(program.statements.len(), 1);
    if let Some(expression_statement) = program.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStatement>()
    {
        if let Some(integer_literal) = expression_statement
            .expression
            .as_any()
            .downcast_ref::<IntegerLiteral>()
        {
            assert_eq!(integer_literal.value, 5);
            assert_eq!(integer_literal.token_literal(), "5");
        } else {
            panic!("Expected: Identifier\nGot: {:?}", expression_statement);
        }
    } else {
        panic!(
            "Expected: ExpressionStatement\nGot: {:?}",
            program.statements[0]
        );
    }
}

#[test]
fn test_parsing_prefix_expressions() {
    #[derive(Debug)]
    struct Test {
        input: Vec<u8>,
        operator: u8,
        integer_value: i64,
    }
    let tests = vec![
        Test {
            input: b"!5;".to_vec(),
            operator: b'!',
            integer_value: 5,
        },
        Test {
            input: b"-15".to_vec(),
            operator: b'-',
            integer_value: 15,
        },
    ];

    for test in tests {
        let lexer = Lexer::new(&test.input);
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse_program().unwrap();

        assert_eq!(program.statements.len(), 1);
        if let Some(expression_statement) = program.statements[0]
            .as_any()
            .downcast_ref::<ExpressionStatement>()
        {
            if let Some(prefix_expression) = expression_statement
                .expression
                .as_any()
                .downcast_ref::<PrefixExpression>()
            {
                assert_eq!(prefix_expression.operator, test.operator);
                test_integer_literal(&*prefix_expression.right, test.integer_value);
            } else {
                panic!(
                    "Expected: PrefixExpression\nGot: {:?}",
                    expression_statement
                );
            }
        } else {
            panic!(
                "Expected: ExpressionStatement\nGot: {:?}",
                program.statements[0]
            );
        }
    }
}

#[test]
fn test_parsing_infix_expression() {
    #[derive(Debug)]
    struct Test {
        input: Vec<u8>,
        left_value: i64,
        operator: Vec<u8>,
        right_value: i64,
    }
    let tests = vec![
        Test {
            input: b"5 + 5;".to_vec(),
            left_value: 5,
            operator: b"+".to_vec(),
            right_value: 5,
        },
        Test {
            input: b"5 - 5;".to_vec(),
            left_value: 5,
            operator: b"-".to_vec(),
            right_value: 5,
        },
        Test {
            input: b"5 * 5;".to_vec(),
            left_value: 5,
            operator: b"*".to_vec(),
            right_value: 5,
        },
        Test {
            input: b"5 / 5;".to_vec(),
            left_value: 5,
            operator: b"/".to_vec(),
            right_value: 5,
        },
        Test {
            input: b"5 > 5;".to_vec(),
            left_value: 5,
            operator: b">".to_vec(),
            right_value: 5,
        },
        Test {
            input: b"5 < 5;".to_vec(),
            left_value: 5,
            operator: b"<".to_vec(),
            right_value: 5,
        },
        Test {
            input: b"5 == 5;".to_vec(),
            left_value: 5,
            operator: b"==".to_vec(),
            right_value: 5,
        },
        Test {
            input: b"5 != 5;".to_vec(),
            left_value: 5,
            operator: b"!=".to_vec(),
            right_value: 5,
        },
    ];

    for test in tests {
        let lexer = Lexer::new(&test.input);
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse_program().unwrap();

        assert_eq!(program.statements.len(), 1);
        if let Some(expression_statement) = program.statements[0]
            .as_any()
            .downcast_ref::<ExpressionStatement>()
        {
            if let Some(inifix_expression) = expression_statement
                .expression
                .as_any()
                .downcast_ref::<InfixExpression>()
            {
                assert_eq!(inifix_expression.operator, test.operator);
                test_integer_literal(&*inifix_expression.left, test.left_value);
                test_integer_literal(&*inifix_expression.right, test.right_value);
            } else {
                panic!(
                    "Expected: PrefixExpression\nGot: {:?}",
                    expression_statement
                );
            }
        } else {
            panic!(
                "Expected: ExpressionStatement\nGot: {:?}",
                program.statements[0]
            );
        }
    }
}

#[test]
fn test_operator_precedence_parsing() {
    struct Test {
        input: Vec<u8>,
        expected: String,
    }
    let tests: Vec<Test> = vec![
        Test {
            input: b"-a * b".to_vec(),
            expected: String::from("((-a) * b)"),
        },
        Test {
            input: b"!-a".to_vec(),
            expected: String::from("(!(-a))"),
        },
        Test {
            input: b"a + b + c".to_vec(),
            expected: String::from("((a + b) + c)"),
        },
        Test {
            input: b"a + b - c".to_vec(),
            expected: String::from("((a + b) - c)"),
        },
        Test {
            input: b"a * b * c".to_vec(),
            expected: String::from("((a * b) * c)"),
        },
        Test {
            input: b"a * b / c".to_vec(),
            expected: String::from("((a * b) / c)"),
        },
        Test {
            input: b"a + b / c".to_vec(),
            expected: String::from("(a + (b / c))"),
        },
        Test {
            input: b"a + b * c + d / e - f".to_vec(),
            expected: String::from("(((a + (b * c)) + (d / e)) - f)"),
        },
        Test {
            input: b"3 + 4; -5 * 5".to_vec(),
            expected: String::from("(3 + 4)((-5) * 5)"),
        },
        Test {
            input: b"5 > 4 == 3 < 4".to_vec(),
            expected: String::from("((5 > 4) == (3 < 4))"),
        },
        Test {
            input: b"5 < 4 != 3 > 4".to_vec(),
            expected: String::from("((5 < 4) != (3 > 4))"),
        },
        Test {
            input: b"3 + 4 * 5 == 3 * 1 + 4 * 5".to_vec(),
            expected: String::from("((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"),
        },
        Test {
            input: b"true".to_vec(),
            expected: String::from("true"),
        },
        Test {
            input: b"false".to_vec(),
            expected: String::from("false"),
        },
        Test {
            input: b"3 > 5 == false".to_vec(),
            expected: String::from("((3 > 5) == false)"),
        },
        Test {
            input: b"3 < 5 == true".to_vec(),
            expected: String::from("((3 < 5) == true)"),
        },
        Test {
            input: b"1 + (2 + 3) + 4".to_vec(),
            expected: String::from("((1 + (2 + 3)) + 4)"),
        },
        Test {
            input: b"(5 + 5) * 2".to_vec(),
            expected: String::from("((5 + 5) * 2)"),
        },
        Test {
            input: b"2 / (5 + 5)".to_vec(),
            expected: String::from("(2 / (5 + 5))"),
        },
        Test {
            input: b"-(5 + 5)".to_vec(),
            expected: String::from("(-(5 + 5))"),
        },
        Test {
            input: b"!(true == true)".to_vec(),
            expected: String::from("(!(true == true))"),
        },
        Test {
            input: b"a + add(b * c) + d".to_vec(),
            expected: String::from("((a + add((b * c))) + d)"),
        },
        Test {
            input: b"add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))".to_vec(),
            expected: String::from("add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))"),
        },
        Test {
            input: b"add(a + b + c * d / f + g)".to_vec(),
            expected: String::from("add((((a + b) + ((c * d) / f)) + g))"),
        },
        // Test {
        //     input: b"a * [1, 2, 3, 4][b * c] * d".to_vec(),
        //     expected: String::from("((a * ([1, 2, 3, 4][(b * c)])) * d)"),
        // },
        // Test {
        //     input: b"add(a * b[2], b[1], 2 * [1, 2][1])".to_vec(),
        //     expected: String::from("add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))"),
        // },
    ];

    for test in tests {
        let lexer = Lexer::new(&test.input);
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse_program().unwrap();
        dbg!(format!("{}", &program));
        assert_eq!(format!("{}", program), test.expected);
    }
}

#[test]
fn test_boolean_expression() {
    struct Test {
        input: Vec<u8>,
        expected: bool,
    }
    let tests: Vec<Test> = vec![
        Test {
            input: b"true;".to_vec(),
            expected: true,
        },
        Test {
            input: b"false;".to_vec(),
            expected: false,
        },
    ];

    for test in tests {
        let lexer = Lexer::new(&test.input);
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse_program().unwrap();

        assert_eq!(program.statements.len(), 1);
        if let Some(expression_statement) = program.statements[0]
            .as_any()
            .downcast_ref::<ExpressionStatement>()
        {
            if let Some(boolean) = expression_statement
                .expression
                .as_any()
                .downcast_ref::<Boolean>()
            {
                assert_eq!(boolean.value, test.expected);
            } else {
                panic!(
                    "Expected: PrefixExpression\nGot: {:?}",
                    expression_statement
                );
            }
        } else {
            panic!(
                "Expected: ExpressionStatement\nGot: {:?}",
                program.statements[0]
            );
        }
    }
}

#[test]
fn test_if_expression() {
    let input = b"if (x < y) { x }";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();
    dbg!(&program);
    assert_eq!(program.statements.len(), 1);
    if let Some(expression_statement) = program.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStatement>()
    {
        if let Some(if_expression) = expression_statement
            .expression
            .as_any()
            .downcast_ref::<IfExpression>()
        {
            let condition = match if_expression
                .condition
                .as_any()
                .downcast_ref::<InfixExpression>()
            {
                Some(i) => i,
                None => panic!(
                    "The condition isn't an InfixExpression\nGot: {}",
                    if_expression.condition
                ),
            };
            let con_left = match condition.left.as_any().downcast_ref::<Identifier>() {
                Some(i) => i,
                None => panic!("Not an Identifier\nGot: {}", condition.left),
            };
            let con_right = match condition.right.as_any().downcast_ref::<Identifier>() {
                Some(i) => i,
                None => panic!("Not an Identifier\nGot: {}", condition.right),
            };
            assert_eq!(con_left.value, String::from("x"));
            assert_eq!(condition.operator, b"<");
            assert_eq!(con_right.value, String::from("y"));
            assert_eq!(if_expression.consequence.statements.len(), 1);
            let statement = match if_expression.consequence.statements[0]
                .as_any()
                .downcast_ref::<ExpressionStatement>()
            {
                Some(e) => e,
                None => panic!(
                    "Expected: ExpressionStatement\nGot: {}",
                    if_expression.consequence.statements[0]
                ),
            };
            let statement = match statement.expression.as_any().downcast_ref::<Identifier>() {
                Some(i) => i,
                None => panic!("Not an Identifier\nGot: {}", statement.expression),
            };
            assert_eq!(statement.value, String::from("x"));
            match if_expression.alternative {
                None => (),
                _ => panic!("The alternative statements where not None"),
            }
        } else {
            panic!("Expected: IfExpression\nGot: {:?}", expression_statement);
        }
    } else {
        panic!(
            "Expected: ExpressionStatement\nGot: {:?}",
            program.statements[0]
        );
    }
}

#[test]
fn test_if_else_expression() {
    let input = b"if (x < y) { x } else { y }";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();
    dbg!(&program);
    assert_eq!(program.statements.len(), 1);
    if let Some(expression_statement) = program.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStatement>()
    {
        if let Some(if_expression) = expression_statement
            .expression
            .as_any()
            .downcast_ref::<IfExpression>()
        {
            let condition = match if_expression
                .condition
                .as_any()
                .downcast_ref::<InfixExpression>()
            {
                Some(i) => i,
                None => panic!(
                    "The condition isn't an InfixExpression\nGot: {}",
                    if_expression.condition
                ),
            };
            let con_left = match condition.left.as_any().downcast_ref::<Identifier>() {
                Some(i) => i,
                None => panic!("Not an Identifier\nGot: {}", condition.left),
            };
            let con_right = match condition.right.as_any().downcast_ref::<Identifier>() {
                Some(i) => i,
                None => panic!("Not an Identifier\nGot: {}", condition.right),
            };
            assert_eq!(con_left.value, String::from("x"));
            assert_eq!(condition.operator, b"<");
            assert_eq!(con_right.value, String::from("y"));
            assert_eq!(if_expression.consequence.statements.len(), 1);
            let statement = match if_expression.consequence.statements[0]
                .as_any()
                .downcast_ref::<ExpressionStatement>()
            {
                Some(e) => e,
                None => panic!(
                    "Expected: ExpressionStatement\nGot: {}",
                    if_expression.consequence.statements[0]
                ),
            };
            let statement = match statement.expression.as_any().downcast_ref::<Identifier>() {
                Some(i) => i,
                None => panic!("Not an Identifier\nGot: {}", statement.expression),
            };
            assert_eq!(statement.value, String::from("x"));
            let else_block = match &if_expression.alternative {
                Some(e) => e,
                None => panic!("Alternative statements are None"),
            };
            assert_eq!(else_block.statements.len(), 1);
            let else_statement = match else_block.statements[0]
                .as_any()
                .downcast_ref::<ExpressionStatement>()
            {
                Some(e) => e,
                None => panic!(
                    "Expected: ExpressionStatement\nGot: {}",
                    else_block.statements[0]
                ),
            };
            let else_statement = match else_statement
                .expression
                .as_any()
                .downcast_ref::<Identifier>()
            {
                Some(i) => i,
                None => panic!("Expected: Identifier\nGot: {}", else_statement.expression),
            };
            assert_eq!(else_statement.value, String::from("y"));
        } else {
            panic!("Expected: IfExpression\nGot: {:?}", expression_statement);
        }
    } else {
        panic!(
            "Expected: ExpressionStatement\nGot: {:?}",
            program.statements[0]
        );
    }
}

#[test]
fn test_function_literal_parsing() {
    let input = b"fn(x, y) { x + y; }";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();
    dbg!(&program);
    assert_eq!(program.statements.len(), 1);
    if let Some(expression_statement) = program.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStatement>()
    {
        if let Some(func_literal) = expression_statement
            .expression
            .as_any()
            .downcast_ref::<FunctionLiteral>()
        {
            assert_eq!(func_literal.parameters.len(), 2);
            assert_eq!(func_literal.parameters[0].value, String::from("x"));
            assert_eq!(func_literal.parameters[1].value, String::from("y"));
            assert_eq!(func_literal.body.statements.len(), 1);
            let statement = match func_literal.body.statements[0]
                .as_any()
                .downcast_ref::<ExpressionStatement>()
            {
                Some(fl) => fl,
                None => panic!(
                    "Expected: ExpressionStatement\nGot: {:?}",
                    program.statements[0]
                ),
            };
            let body = match statement
                .expression
                .as_any()
                .downcast_ref::<InfixExpression>()
            {
                Some(i) => i,
                None => panic!(
                    "The condition isn't an InfixExpression\nGot: {}",
                    statement.expression
                ),
            };
            let body_left = match body.left.as_any().downcast_ref::<Identifier>() {
                Some(i) => i,
                None => panic!("Not an Identifier\nGot: {}", body.left),
            };
            assert_eq!(body_left.value, "x");
            assert_eq!(body.operator, b"+");
            let body_right = match body.right.as_any().downcast_ref::<Identifier>() {
                Some(i) => i,
                None => panic!("Not an Identifier\nGot: {}", body.right),
            };
            assert_eq!(body_right.value, "y");
        } else {
            panic!("Expected: IfExpression\nGot: {:?}", expression_statement);
        }
    } else {
        panic!(
            "Expected: ExpressionStatement\nGot: {:?}",
            program.statements[0]
        );
    }
}

#[test]
fn test_function_parameter_parsing() {
    struct Test {
        input: Vec<u8>,
        expected_params: Vec<u8>,
    }
    let tests: Vec<Test> = vec![
        Test {
            input: b"fn() {};".to_vec(),
            expected_params: b"".to_vec(),
        },
        Test {
            input: b"fn(x) {};".to_vec(),
            expected_params: b"x".to_vec(),
        },
        Test {
            input: b"fn(x, y, z) {};".to_vec(),
            expected_params: b"xyz".to_vec(),
        },
    ];

    for test in tests {
        let lexer = Lexer::new(&test.input);
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse_program().unwrap();
        if let Some(expression_statement) = program.statements[0]
            .as_any()
            .downcast_ref::<ExpressionStatement>()
        {
            if let Some(function) = expression_statement
                .expression
                .as_any()
                .downcast_ref::<FunctionLiteral>()
            {
                assert_eq!(function.parameters.len(), test.expected_params.len());
                for (i, ident) in test.expected_params.iter().enumerate() {
                    let param = match function.parameters[i].as_any().downcast_ref::<Identifier>() {
                        Some(i) => i,
                        None => panic!("Expected: Identifier\nGot: {}", function.parameters[i]),
                    };
                    assert_eq!(&param.value.as_bytes()[0], ident);
                }
            } else {
                panic!(
                    "Expected: FunctionLiteral\nGot: {:?}",
                    expression_statement.expression
                );
            }
        } else {
            panic!(
                "Expected: ExpressionStatement\nGot: {:?}",
                program.statements[0]
            );
        }
    }
}

#[test]
fn test_call_expression_parsing() {
    let input = b"add(1, 2 * 3, 4 + 5);";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();
    assert_eq!(program.statements.len(), 1);
    if let Some(expression_statement) = program.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStatement>()
    {
        if let Some(call_expression) = expression_statement
            .expression
            .as_any()
            .downcast_ref::<CallExpression>()
        {
            let func = match call_expression
                .function
                .as_any()
                .downcast_ref::<Identifier>()
            {
                Some(f) => f,
                None => panic!("Expected: Identifier\nGot: {:?}", call_expression.function),
            };
            assert_eq!(func.value, String::from("add"));
            assert_eq!(call_expression.arguments.len(), 3);
            let first_arg = match call_expression.arguments[0]
                .as_any()
                .downcast_ref::<IntegerLiteral>()
            {
                Some(i) => i,
                None => panic!(
                "Not the right kind of Expression. Expected: Expression::IntegerLiteral\nGot: {}",
                    call_expression.arguments[0]
            ),
            };
            assert_eq!(first_arg.value, 1);
            let second_arg = match call_expression.arguments[1]
                .as_any()
                .downcast_ref::<InfixExpression>()
            {
                Some(i) => i,
                None => panic!(
                "Not the right kind of Expression. Expected: Expression::InfixExpression\nGot: {}",
                    call_expression.arguments[1]
            ),
            };
            let second_left = match second_arg.left.as_any().downcast_ref::<IntegerLiteral>() {
                Some(i) => i,
                None => panic!(
                    "Not the right kind of Expression. Expected: Expression::Identifier\nGot: {}",
                    second_arg.left
                ),
            };
            let second_right = match second_arg.right.as_any().downcast_ref::<IntegerLiteral>() {
                Some(i) => i,
                None => panic!(
                    "Not the right kind of Expression. Expected: Expression::Identifier\nGot: {}",
                    second_arg.right
                ),
            };
            assert_eq!(second_left.value, 2);
            assert_eq!(second_arg.operator, b"*");
            assert_eq!(second_right.value, 3);
            let third_arg = match call_expression.arguments[2]
                .as_any()
                .downcast_ref::<InfixExpression>()
            {
                Some(i) => i,
                None => panic!(
                "Not the right kind of Expression. Expected: Expression::InfixExpression\nGot: {}",
                    call_expression.arguments[2]
            ),
            };
            let third_left = match third_arg.left.as_any().downcast_ref::<IntegerLiteral>() {
                Some(i) => i,
                None => panic!(
                    "Not the right kind of Expression. Expected: Expression::Identifier\nGot: {}",
                    third_arg.left
                ),
            };
            let third_right = match third_arg.right.as_any().downcast_ref::<IntegerLiteral>() {
                Some(i) => i,
                None => panic!(
                    "Not the right kind of Expression. Expected: Expression::Identifier\nGot: {}",
                    third_arg.left
                ),
            };
            assert_eq!(third_left.value, 4);
            assert_eq!(third_arg.operator, b"+");
            assert_eq!(third_right.value, 5);
        } else {
            panic!(
                "Expected: CallExpression\nGot: {:?}",
                expression_statement.expression
            );
        }
    } else {
        panic!(
            "Expected: ExpressionStatement\nGot: {:?}",
            program.statements[0]
        );
    }
}
