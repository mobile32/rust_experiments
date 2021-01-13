fn convert_to_pig_latin(word: &str) -> String {
    let vovels = "aeiou";

    let first_word_letter = word.chars().next().unwrap();
    for vovel in vovels.chars() {
        if vovel == first_word_letter {
            return word.to_string() + "hay";
        }
    }

    word[1..word.len()].to_string() + &first_word_letter.to_string() + "ay"
}
