use npc::convertor::NamingPrincipalConvertor;

fn main() {
    let source = "snake_case";
    let npc = NamingPrincipalConvertor::new(source);
    assert_eq!(npc.to_snake(), String::from("snake_case"));
    assert_eq!(npc.to_camel(), String::from("snakeCase"));
    assert_eq!(npc.to_pascal(), String::from("SnakeCase"));
    assert_eq!(npc.to_chain(), String::from("snake-case"));
    assert_eq!(npc.to_constant(), String::from("SNAKE_CASE"));
}
