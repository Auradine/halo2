use halo2_gadgets::sha256_input::get_input;

#[test]
fn test_sha256_main() {
    let input = get_input();
    println!("input({}): {:?}", input.len(), &input);
}
