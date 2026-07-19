// Calls of this function should be replaced with calls of `string_slice` or `string`.
enum Text {
    Slice(&'static str),
    Owned(String),
}

fn placeholder(text: Text) {
    match text {
        Text::Owned(text) => string(text),
        Text::Slice(text) => string_slice(text),
    }
}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    placeholder(Text::Slice("blue"));

    placeholder(Text::Owned("red".to_string()));

    placeholder(Text::Owned(String::from("hi")));

    placeholder(Text::Owned("rust is fun!".to_owned()));

    placeholder(Text::Owned("nice weather".into()));

    placeholder(Text::Owned(format!("Interpolation {}", "Station")));

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    placeholder(Text::Slice(&"abc"[0..1]));

    placeholder(Text::Slice("  hello there ".trim()));

    placeholder(Text::Owned("Happy Monday!".replace("Mon", "Tues")));

    placeholder(Text::Owned("mY sHiFt KeY iS sTiCkY".to_lowercase()));
}
