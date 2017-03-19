
#![allow(dead_code)]

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
    pub fn welcome_messages() -> Vec<String> {
        vec!(
            "Now what?".to_string(),
            "and then you.., what exactly?".to_string(),
            "Will you rescue yourself or save the kitten?".to_string(),
            "I'm certain that I put a treasure somewhere.".to_string(),
            "This might be confusing for you, but try something out maybe".to_string(),
            "Try to figure this one out, muhahaha         ha.".to_string(),
            "Hello, what do you want do to?".to_string(),
        )
    }

    pub fn quit_messages() -> Vec<String> {
        vec!(
            "Back to work peasant!".to_string(),
            "It's not too late, I'll give you a second chance! ... on second thought, I can't bother, goodbye quitter".to_string(),
            "Didn't you want to save the world or something?".to_string(),
            "However you got in here, you found the exit (or so you believe)".to_string(),
            "Did you know I prepared a cake? Ahwell, there will be others".to_string(),
            "You are tired, I understand".to_string(),
            "Your trip on a stick and then crawl away into a ditch".to_string(),
            "You drop everything you carry and run for your life".to_string(),
            )
    }

    pub fn help_messages() -> Vec<String> {
        vec!(
            "Just try something, anything".to_string(),
            "You think that you deserve help? What did you ever help me with?".to_string(),
            "I recommend the most pouplar option to just quit".to_string(),
            "There there, try to calm down. Screaming will not help you.".to_string(),
            "I could tell you how to win, but would you trust me?".to_string(),
            )
    }

    pub fn what_messages() -> Vec<String> {
        vec!(
            "I understand, but maybe you should search the room first".to_string(),
            "You can always give up and quit instead".to_string(),
            "Don't be so needy".to_string(),
            "You thought I had time to implement that?! think again".to_string(),
            "Maybe use one of your items".to_string(),
            )
    }
}
