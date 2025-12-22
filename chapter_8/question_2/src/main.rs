use std::io;

enum Letter {
    Vowel,
    Consonant
}

fn get_type(s: &char) -> Letter {
    match s {
       'a' => Letter::Vowel, 
       'e' => Letter::Vowel,
       'i' => Letter::Vowel,
       'o' => Letter::Vowel,
       'u' => Letter::Vowel,
       _ => Letter::Consonant
    }
}

fn to_pig_latin(s: &String) -> String {
    let mut pig_str = String::new();

    for word in s.split(' ') {
        let first_letter = word.chars().nth(0).unwrap();
        match get_type(&first_letter) {
            Letter::Consonant => {
                let (first_letter, rest_word) = word.split_at(1);
                pig_str.push_str(rest_word);
                pig_str.push_str("-");
                pig_str.push_str(first_letter);
                pig_str.push_str("ay ");
            }, 
            Letter::Vowel => {
                pig_str.push_str(word);
                pig_str.push_str("-hay ");
            },
        }
    }
    
    pig_str.pop();
    pig_str
}

fn main() {
    let mut original = String::new();
    io::stdin().read_line(&mut original).unwrap();
    original = original.trim_end().to_string();
    let pig_str = to_pig_latin(&original);
    println!("{pig_str}");
}
