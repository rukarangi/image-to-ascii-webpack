/*
This file contains all the possible effects that can be chosen.
*/

fn filter(scalar: f64, characters: Vec<char>) -> char {
    let mut character: char = ' ';

    let length = characters.len();
    let difference: f64 = 255.0 / length as f64;
    for (i, chara) in characters.iter().enumerate() {
        let below: f64 = difference * (i + 1) as f64; 

        if scalar < below {
            character = *chara;
            break;
        }
    }

    return character;
}

pub fn grayscale_basic(rgb: (f64, f64, f64, f64), reverse: bool) -> char {
    let grayscale_default: Vec<char> = vec![' ','.',':','-','=','+','*','#','%','@'];
    let grayscale: Vec<char>;

    let average = (rgb.0 + rgb.1 + rgb.2) / 3.0;

    // reverse vector of characters if chosen
    if reverse {
        grayscale = grayscale_default.into_iter().rev().collect();
    } else {
        grayscale = grayscale_default;
    }

    return filter(average, grayscale);
}

pub fn grayscale_basic_test(gray: f64, reverse: bool) -> char {
    let grayscale_default: Vec<char> = vec![' ','.',':','-','=','+','*','#','%','@'];
    let grayscale: Vec<char>;

    let average = gray;

    // reverse vector of characters if chosen
    if reverse {
        grayscale = grayscale_default.into_iter().rev().collect();
    } else {
        grayscale = grayscale_default;
    }

    return filter(average, grayscale);
}

pub fn grayscale_detailed(rgb: (f64, f64, f64), reverse: bool) -> char {
    let grayscale_default: Vec<char> = vec!['$','@','B','%','8','&','W','M','#','*','o','a','h','k','b','d','p','q','w','m','Z','O','0','Q','L','C','J','U','Y','X','z','c','v','u','n','x','r','j','f','t','/','|','(',')','1','{','}','[',']','?','-','_','+','~','<','>','i','!','l','I',';',':',',','^','`','.',',',' '];
    let grayscale: Vec<char>;

    let average = (rgb.0 + rgb.1 + rgb.2) / 3.0;

    // reverse vector of characters if chosen
    if reverse {
        grayscale = grayscale_default.into_iter().rev().collect();
    } else {
        grayscale = grayscale_default;
    }

    return filter(average, grayscale);
}

pub fn redgreen_basic(rgb: (f64, f64, f64), reverse: bool) -> char {
    let grayscale_default: Vec<char> = vec![' ','.',':','-','=','+','*','#','%','@'];
    let grayscale: Vec<char>;

    let average = (rgb.0 + (rgb.1 / 2.0)) / 1.5;

    // reverse vector of characters if chosen
    if reverse {
        grayscale = grayscale_default.into_iter().rev().collect();
    } else {
        grayscale = grayscale_default;
    }

    return filter(average, grayscale);
}