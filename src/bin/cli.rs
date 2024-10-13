use npc::*;

fn main() {
    let param = Parameter::new("HelloWorld!", npc::Principal::Snake);
    println!("{}", convert(&param));

    let param = param.change_principal(npc::Principal::Camel);
    println!("{}", convert(&param));

    let param = param.change_principal(npc::Principal::Pascal);
    println!("{}", convert(&param));

    let param = param.change_principal(npc::Principal::Constant);
    println!("{}", convert(&param));

    let param = param.change_principal(npc::Principal::Chain);
    println!("{}", convert(&param));
}
