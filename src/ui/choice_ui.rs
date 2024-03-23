use std::io;

pub struct Choice<Args, Res> {
    prompt: String,
    callback: Box<dyn Fn(&mut Args) -> Res>
}

impl<Args, Res> Choice<Args, Res> {
    pub fn new(prompt: String, callback: Box<dyn Fn(&mut Args) -> Res>) -> Self {
        Choice {prompt, callback}
    }
    
    pub fn call(&self, args: &mut Args) -> Res {
        (self.callback)(args)
    }
}

pub fn ui_offer_choices<Args, Res>(choices: &Vec<Choice<Args, Res>>, args: &mut Args) -> Option<Res> {
    for (index, choice) in choices.iter().enumerate() {
        println!("\t{}. {}", index + 1, choice.prompt)
    }
    println!("\tq. Quit");
    let mut buf = String::new();
    loop {
        buf.clear();
        io::stdin().read_line(&mut buf).expect("Failed to read user input!");
        let buf = buf.trim();
        if buf == "q" {
            return None;
        }
        let choice_number = buf.parse::<usize>().expect("failed to parse user input as number!") - 1;
        if choice_number < choices.len() {
            return Some(choices[choice_number].call(args));
        }
        else {
            println!("{} was not recognized as an available option! Try again or press 'q' to quit!", choice_number)
        }
    }
}