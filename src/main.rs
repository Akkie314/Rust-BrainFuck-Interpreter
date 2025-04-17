use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("読み込むファイル名を入力してください");
    let mut filename = String::new();
    std::io::stdin().read_line(&mut filename).unwrap();
    let filename = filename.trim();
    let code = read_file(filename);
    println!("読み込んだコード: {}", code);
    interpriter(&code);
}

fn read_file(filepath: &str) -> String {
    let file = File::open(filepath).expect("Unable to open file");
    let mut contents = String::new();
    std::io::BufReader::new(file)
        .read_to_string(&mut contents)
        .expect("Unable to read file");
    return contents;
}

fn interpriter(code : &str) {
    // メモリを初期化
    let mut memory: [u8; 30000] = [0; 30000];
    let mut pointer: usize = 0;

    // ループスタック
    let mut loop_stack: Vec<usize> = Vec::new();

    // ループカウント
    let mut loop_count: HashMap<usize, usize> = HashMap::new();
    let loop_count_max: usize = 10000;

    // コードを1文字ずつ処理
    let code: Vec<char> = code.chars().collect();
    let mut code_pointer: usize = 0;

    loop {
        // コードを解読し終わったら終了
        if code_pointer >= code.len() {
            break;
        }

        let c = code[code_pointer];
        match c {
            '+' => {
                memory[pointer] = memory[pointer].wrapping_add(1);

                code_pointer += 1;
            },
            '-' => {
                memory[pointer] = memory[pointer].wrapping_sub(1);

                code_pointer += 1;
            },
            '>' => {
                pointer += 1;

                // メモリの外側にアクセスしようとしたらパニック
                if pointer >= memory.len() {
                    panic!("Pointer out of bounds");
                }

                code_pointer += 1;
            },
            '<' => {
                pointer -= 1;

                // メモリの外側にアクセスしようとしたらパニック
                if pointer >= memory.len() {
                    panic!("Pointer out of bounds");
                }

                code_pointer += 1;
            },
            '.' => {
                // メモリの値を出力
                let value = memory[pointer];
                let c = u8_to_char(value);
                print!("{}", c);

                code_pointer += 1;
            },
            ',' => {
                // 標準入力から1バイト読み込む
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let value = input.trim().chars().next().unwrap_or('\0');
                memory[pointer] = char_to_u8(value);

                code_pointer += 1;
            },
            '[' => {
                loop_stack.push(code_pointer);

                // ループカウントを初期化
                loop_count.insert(code_pointer, 0);

                code_pointer += 1;
            },
            ']' => {
                if memory[pointer] == 0 {
                    loop_stack.pop();
                    code_pointer += 1;
                } else {
                    let loop_start = *loop_stack.last().unwrap();
                    loop_count.insert(loop_start, loop_count[&loop_start] + 1);
                    if loop_count[&loop_start] > loop_count_max {
                        panic!("Loop count exceeded");
                    }
                    code_pointer = loop_start;
                }
            },
            _ => {
                // 無視する
                code_pointer += 1;
            }
        }
    }
}

fn u8_to_char(value: u8) -> char {
    // u8をu32に変換
    let value = value as u32;

    // u32をascii変換
    char::from_u32(value).unwrap_or('\0')
}

fn char_to_u8(value: char) -> u8 {
    let value = value as u32;
    if value < 128 {
        return value as u8;
    } else {
        return 0;
    }
}