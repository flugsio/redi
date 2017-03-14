
pub struct Phrase {
    pub message: String,
}

pub struct Phrases {
}

impl Phrase {
    fn new(message: &str) -> Phrase {
        Phrase {
            message: message.to_string(),
        }
    }
}

impl Phrases {
    pub fn random_welcome() -> String {
        [
            "Now what?",
            "and then you.., what exactly?",
            "Will you rescue yourself or save the kitten?",
            "I'm certain that I put a treasure somewhere.",
            "This might be confusing for you, but try something out maybe",
            "Try to figure this one out, muhahaha         ha.",
            "Hello, what do you want do to?",
        ][1].to_string()
    }

    pub fn random_ragequit() -> String {
        [
            "Back to work peasant!!",
            "It's not too late, I'll give you a second chance! ... on second thought, I can't bother, goodbye quitter",
            "Didn't you want to save the world or something?",
            "However you got in here, you found the exit (or so you believe)",
            "Did you know I prepared a cake? Ahwell, there will be others",
            "You are tired, I understand",
            "Your trip on a stick and then crawl away into a ditch",
            "You drop everything you carry and run for your life",
        ][0].to_string()
    }

    // TODO: implement random somehow
}
