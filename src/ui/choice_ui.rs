use std::io;

pub struct Choice<'a, Args, Res> {
    prompt: String,
    callback: Box<dyn FnOnce(Args) -> Res + 'a>
}

impl<'a, Args, Res> Choice<'a, Args, Res> {
    pub fn new(prompt: String, callback: Box<dyn FnOnce(Args) -> Res + 'a>) -> Self {
        Choice {prompt, callback}
    }
    
    pub fn call(self, args: Args) -> Res {
        (self.callback)(args)
    }
}

pub fn ui_offer_choices<Args, Res>(mut choices: Vec<Choice<Args, Res>>, args: Args) -> Option<Res> {
    let choice = ui_get_user_choice(&mut choices);
    match choice {
        Some(choice) => {
            println!("Selected {}", choice.prompt);
            Some(choice.call(args))
        }
        None => {
            println!("quiting...");
            None
        }
    }
}

pub fn ui_get_user_choice<'a, Args, Res>(choices: &mut Vec<Choice<'a, Args, Res>>) -> Option<Choice<'a, Args, Res>> {
    for (index, choice) in choices.iter().enumerate() {
        println!("\t{}. {}", index + 1, choice.prompt)
    }
    println!("\tq. Quit Selection");
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
            return Some(choices.swap_remove(choice_number));
        }
        else {
            println!("{} was not recognized as an available option! Try again or press 'q' to quit!", choice_number)
        }
    }
}