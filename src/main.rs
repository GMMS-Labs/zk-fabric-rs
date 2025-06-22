mod psg; // import the psg module

use crate::psg::polylithic_syntax_gen;

fn main() {
    let input = r#"The car only starts if the "start" button is pressed and the "brake" pedal is pressed"#;

    let circuit = polylithic_syntax_gen(input);

    println!("Generated Boolean Circuit:\n{:#?}", circuit);
}
