#[derive(Debug)]
pub enum NumberSystem<'a> {
    Binary(&'a String),
    // Octal(String),
    Decimal(&'a String),
    // Hexadecimal(String),
}

pub trait NumberSystemTrait {
    fn to_binary(&self) -> String;
    fn to_octal(&self) -> u32;
    fn to_decimal(&self) -> u32;
    fn to_hexadecimal(&self) -> String;
}


fn binary_to_decimal(binary:&String) -> u32{
    let mut _decimal_value:u32 = 0;
    let trimmed_binary: String = binary.trim_start_matches('0').chars().collect();
    let binary_len = trimmed_binary.len() -1;
    for (index, character) in trimmed_binary.char_indices() {
        if character == '1'{
            _decimal_value += 2u32.pow((binary_len - index) as u32);
        }
    }
    _decimal_value
}

fn binary_chunks_to_decimal(binary:&String, base:usize) -> String{
    let _mod:u8 = (binary.len() % base) as u8;
    let fill_count:u8 = if _mod == 0 { 0 } else { (base as u8 )- _mod };
    let filled_value:String = "0".repeat(fill_count as usize) + binary;
    let mut converted_value:String = String::from("");

    for chunk in filled_value.chars().collect::<Vec<char>>().chunks(base){
        let chunk_binary: String = chunk.into_iter().collect();
        let decimal_chunk:String = binary_to_decimal(&chunk_binary).to_string();
        if base == 4 {
            let hexa:String = hexa_char(&decimal_chunk);
            converted_value += &hexa;
        }else{
            converted_value += &decimal_chunk
        }
    }

    converted_value
}

fn hexa_char(value:&str) -> String{
    let hexa =  match value {
        "10" => "A",
        "11" => "B",
        "12" => "C",
        "13" => "D",
        "14" => "E",
        "15" => "F",
        other => other,
    };
    hexa.to_string()
}

fn decimal_to_other_ns(value:&String, base:u32) -> String {
    let decimal_value:u32 = value.parse().unwrap_or(0);
    let mut converted_value:String = String::from("");
    let mut remainder:u32 = decimal_value;

    while remainder >= base {
        let quotient:String = (remainder % base).to_string();
        if base == 16 {
            let hexa:String = hexa_char(&quotient);
            converted_value += &hexa;
        }else{
            converted_value += &quotient
        }
        remainder = remainder / base;
    }

    if base == 16 {
        let hexa:String = hexa_char(&remainder.to_string());
        converted_value += &hexa;
    }else{
        converted_value += remainder.to_string().as_str();
    }
    converted_value.chars().rev().collect()
}


impl NumberSystemTrait for NumberSystem<'_> {
    fn to_binary(&self) -> String {
        match self {
            NumberSystem::Binary(value) => value.to_string(),
            // NumberSystem::Octal(value) => value.to_string(),
            NumberSystem::Decimal(value) => decimal_to_other_ns(value, 2)
            // NumberSystem::Hexadecimal(value) => value.to_string(),
        }
    }

    fn to_octal(&self) -> u32 {
        match self {
            NumberSystem::Binary(value) => {
                let octal_value:String = binary_chunks_to_decimal(value, 3);
                octal_value.parse().unwrap_or(0)
            }
            // NumberSystem::Octal(value) => value.to_string(),
            NumberSystem::Decimal(value) => {
                let octal:String = decimal_to_other_ns(value,8);
                octal.parse().unwrap_or(0)
            }
            // NumberSystem::Hexadecimal(value) => value.to_string(),
        }
    }

    fn to_decimal(&self) -> u32 {
        match self {
            NumberSystem::Binary(value) => binary_to_decimal(value),
            // NumberSystem::Octal(value) => value.to_string(),
            NumberSystem::Decimal(value) => value.parse().unwrap_or(0),
            // NumberSystem::Hexadecimal(value) => value.to_string(),
        }
    }

    fn to_hexadecimal(&self) -> String {
        match self {
            NumberSystem::Binary(value) => binary_chunks_to_decimal(value, 4),
            // NumberSystem::Octal(value) => value.to_string(),
            NumberSystem::Decimal(value) => decimal_to_other_ns(value,16),
            // NumberSystem::Hexadecimal(value) => value.to_string(),
        }
    }
}




