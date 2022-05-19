use std::io;

fn main() {
    let mut pointer: usize = 15000;
    let mut array_byte: [u8; 30000] = [0; 30000];
    let mut code: String = String::new();

    io::stdin()
        .read_line(&mut code)
        .expect("Error: Failed to read code.");
        
    analize_code(&mut code, &mut pointer, &mut array_byte);
}

fn analize_code(code: &mut String, pointer: &mut usize, array_byte: &mut [u8; 30000]) {
    let mut loop_stack: usize = 0;
    let mut loop_code: String = String::new();
    let mut iter_byte: usize = 0;

    for ch in code.chars().collect::<Vec<char>>() {
        if loop_stack != 0 {
            match ch {
                '[' => loop_stack += 1,
                ']' => loop_stack -= 1,
                _ => {}
            }

            if loop_stack == 0 {
                while array_byte[iter_byte] != 0 {
                    analize_code(&mut loop_code, pointer, array_byte);
                }
            } else {
                loop_code.push(ch);
            }
        } else {
            match ch {
                '+' => array_byte[*pointer] += 1,
                '-' => array_byte[*pointer] -= 1,
                '<' => *pointer -= 1,
                '>' => *pointer += 1,
                '.' => print!("{}", array_byte[*pointer] as char),
                '[' => {
                    loop_stack += 1;
                    iter_byte = *pointer;
                },
                _ => {}
            }
        }
    }
}