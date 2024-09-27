use codigo_morse::decode_codigo_morse_1::decode_morse;
fn main() {
    let hey = decode_morse(".... . -.--   .--- ..- -.. .");
    
    println!("{}", hey);
}
