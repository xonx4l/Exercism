fn pig_latin(input: &str) -> String {
     input 
         .split_whitespace()
         .map(translate_word)
         .collect::<Vec<String>>()
         .join("")
}

fn pig_latin_word(word: &str) ->String {
   let n = consonant_chars(word);
   let (consonant, other ) = word.split_at(n);

   format!("{}{}ay",other,consonant)
}

fn consonant_chars(word: &str) -> usize {
     if word.is_empty()
        || word.starts_with('a')
        || word.starts_with('e')
        || word.starts_with('i')
        || word.starts_with('o')
        || word.starts_with('u')
        || word.starts_with("xr")
        || word.starts_with("yt")
{
        0
    } else if word.starts_with("qu") {
        2
    } else if word.chars().nth(1) == Some('y') {
        1
    } else {
        1 + consonant_chars(&word[1..])
    }
}
