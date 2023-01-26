use std::io;
use std::fs;

fn main() {
    println!("******* Memory Match *******");

    loop {
        // get option
        println!("\nOptions:\n  1. Create Quiz\n  2. Load Quiz\n  3. Help\n  4. Quit");
        println!("{}", "  =>");

        let mut option_s = String::new();
        let option_i : i32;
        io::stdin().read_line(&mut option_s).expect("failed to readline");

        match option_s.trim().parse::<i32>() {
            Ok(n) => option_i = n,
            Err(_e) => option_i = 0,
        }

        // act on option
        match option_i {
            1 => create_quiz(),
            2 => load_quiz(),
            3 => show_help(),
            4 => return,
            _ => println!("\n\nInvalid option. Try again."),
        }
    }
}

fn create_quiz() {

    // get name of quiz
    println!("\n\nWhat would you like to name the quiz?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("failed to readline");

    // create struct
    let mut quiz = Quiz {
        name: name.trim().to_string().clone(),
        shtuff: Vec::<(String, String)>::new(),
    };

    // create quiz questions and answers
    loop {
        println!("\n[Enter 'x' at any time to finish quiz creation]");

        println!("Enter a question:");
        let mut question = String::new();
        io::stdin().read_line(&mut question).expect("failed to readline");

        // finish quiz creation
        if question.trim_end().eq("x") {
            break;
        }

        println!("Enter a answer:");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("failed to readline");

        // finish quiz creation
        if answer.trim_end().eq("x") {
            break;
        }

        // update struct
        quiz.shtuff.push((question, answer));
    }

    // create file
    let mut text: String = "".to_string();
    for x in quiz.shtuff {
        text += &("".to_owned() + (&x.0).trim() + "," + (&x.1).trim() + "\n");
    }
    fs::write(quiz.name.clone() + ".txt", text).expect("unable to write to file");
}

fn load_quiz() {

    // prompt for name of quiz
    println!("\n\nWhich quiz would you like to load?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("failed to readline");
    name = name.trim().to_string();
    name = name + ".txt";

    // read in quiz file's contents. return to main program loop if quiz does not exist
    let op = fs::read(name.clone());
    let text: String;
    match op {
        Err(_e) => {println!("\nNo quiz found with that name."); return},
        Ok(t) => text = String::from_utf8_lossy(&t).parse().unwrap(),
    }

    // populate a Quiz struct
    let mut quiz = Quiz {
        name: name.trim().to_string().clone(),
        shtuff: Vec::<(String, String)>::new(),
    };
    let lines = text.lines();
    for x in lines {
        let blah = x.split(",");
        let mut question = "";
        let mut answer = "";
        for (i,y) in blah.enumerate() {
            if i == 0 {
                question = y
            } else {
                answer = y
            }
        }
        quiz.shtuff.push((question.to_string().clone(), answer.to_string().clone()));
    }

    // quiz the user
    let mut num_correct: u32 = 0;
    for x in &quiz.shtuff {
        println!("\n** Question: {}", x.0);

        let mut ans = String::new();
        io::stdin().read_line(&mut ans).expect("failed to readline");
        ans = ans.trim().to_string();

        if ans == x.1 {
            println!("Correct!");
            num_correct += 1;
        } else {
            println!("Incorrect.. The correct answer is: {}", x.1);
        }
    }

    // print the amt of questions answered correctly
    println!("\n[You got {} out of {} correct. Keep up the good work!]", num_correct, quiz.shtuff.len());
}

fn show_help() {
    println!("\n\nWelcome to Memory Match!\nMemory match is a tool to help students study for quizzes.\nGet started by creating your first quiz!");
}

struct Quiz {
    name: String,
    shtuff: Vec<(String, String)>,
}