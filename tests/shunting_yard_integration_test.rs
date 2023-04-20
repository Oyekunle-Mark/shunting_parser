use shunting_parser;

#[test]
fn can_evaluate_expressions_with_simple_operators() {
    assert_eq!(
        4.0,
        shunting_parser::evaluate_expression_shunting_yard("2+2")
    );
}
