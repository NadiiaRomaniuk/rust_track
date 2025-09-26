pub fn reply(message: &str) -> &str {
    let msg = message.trim();

    if msg.is_empty() {
        "Fine. Be that way!"
    } else if msg.ends_with('?') && msg.to_uppercase() == msg && msg.chars().any(|c| c.is_alphabetic()) {
        "Calm down, I know what I'm doing!"
    } else if msg.ends_with('?') {
        "Sure."
    } else if msg.to_uppercase() == msg && msg.chars().any(|c| c.is_alphabetic()) {
        "Whoa, chill out!"
    } else { "Whatever."
    }
}
