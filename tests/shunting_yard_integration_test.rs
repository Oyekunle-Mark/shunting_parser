use shunting_parser;

#[test]
fn can_evaluate_expressions_with_simple_operators() {
    assert_eq!(
        4.0,
        shunting_parser::evaluate_expression_shunting_yard("2+2")
    );
    assert_eq!(
        11.0,
        shunting_parser::evaluate_expression_shunting_yard("3 + 4 * 2 / 1 ^ 2 ^ 3")
    );
}

#[test]
fn can_evaluate_expressions_with_simple_operators_and_parenthesis() {
    assert_eq!(
        3.0001220703125,
        shunting_parser::evaluate_expression_shunting_yard("3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3")
    );
}

#[test]
fn can_evaluate_expressions_with_constants() {
    assert_eq!(
        0.9141592653589999,
        shunting_parser::evaluate_expression_shunting_yard("((2*3) + pi) / 10")
    );
}

#[test]
fn can_evaluate_expressions_with_functions() {
    assert_eq!(
        1.0,
        shunting_parser::evaluate_expression_shunting_yard("min(1,2)")
    );
    assert_eq!(
        14.206,
        shunting_parser::evaluate_expression_shunting_yard("max(14.206, 14.201)")
    );
}

#[test]
fn can_evaluate_expressions_with_simple_operators_and_parenthesis_and_functions() {
    assert_eq!(
        3.0001220703125,
        shunting_parser::evaluate_expression_shunting_yard(
            "3 + 4 * 2 / ( 1 - max(5, 2) ) ^ min(2,11) ^ 3"
        )
    );
}
