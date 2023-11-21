use std::array;
use std::io;
use std::time;
use rand::Rng;
use rand::thread_rng;
use rand::seq::SliceRandom;


fn main() {
    let mut input = String::new();
    let mut word_count: f32 = 0.0;
    let before_time = time::SystemTime::now();
    let mut wpm: f32 = 0.00000;
    let mut target_string = String::new();
    const TARGET_STRING_LEN: i32 = 10;
    let mut rng = thread_rng();
    let mut accuracy = 0.0;

    let words: [&str; 200] = [
    "the", "be", "of", "and", "a", "to", "in", "he", "have", "it", "that", "for", "they", "I", "with",
    "as", "not", "on", "she", "at", "by", "this", "we", "you", "do", "but", "from", "or", "which", "one",
    "would", "all", "will", "there", "say", "who", "make", "when", "can", "more", "if", "no", "man", "out",
    "other", "so", "what", "time", "up", "go", "about", "than", "into", "could", "state", "only", "new", "year",
    "some", "take", "come", "these", "know", "see", "use", "get", "like", "then", "first", "any", "work", "now",
    "may", "such", "give", "over", "think", "most", "even", "find", "day", "also", "after", "way", "many", "must",
    "look", "before", "great", "back", "through", "long", "where", "much", "should", "well", "people", "down",
    "own", "just", "because", "good", "each", "those", "feel", "seem", "how", "high", "too", "place", "little",
    "world", "very", "still", "nation", "hand", "old", "life", "tell", "write", "become", "here", "show", "house",
    "both", "between", "need", "mean", "call", "develop", "under", "last", "right", "move", "thing", "general",
    "school", "never", "same", "another", "begin", "while", "number", "part", "turn", "real", "leave", "might",
    "want", "point", "form", "off", "child", "few", "small", "since", "against", "ask", "late", "home", "interest",
    "large", "person", "end", "open", "public", "follow", "during", "present", "without", "again", "hold", "govern",
    "around", "possible", "head", "consider", "word", "program", "problem", "however", "lead", "system", "set",
    "order", "eye", "plan", "run", "keep", "face", "fact", "group", "play", "stand", "increase", "early", "course",
    "change", "help", "line",
    ];    


    for _ in 0..=TARGET_STRING_LEN {
        if let Some(random_word) = words.choose(&mut rng) {
            target_string += format!("{} ", *random_word).as_str();
        }
    }

    target_string = target_string.trim_end_matches(" ").to_string();

    println!("YOu have to type:\n{}", target_string);
    io::stdin().read_line(&mut input);
    let input = input.trim();

    let time_after = time::SystemTime::now()
        .duration_since(before_time)
        .unwrap()
        .as_secs();




    for char in input.chars() {
        if char == ' ' {
            word_count += 1.0;
        }
    }

    println!("time after {:?}", time_after);

    if input == target_string {
        accuracy = 100.0;
    } else {
        accuracy = rng.gen_range(20.0..80.0);
    }

    println!("input is {input}\n{target_string}.");
    
    wpm = (word_count / time_after as f32) * 60 as f32; 


    println!("You achieved {wpm} WPM with {accuracy}% accuacty.")

}