use std::io;
use std::process;
use std::io::{Write,Read};

// use crate::blockchain;
mod blockchain;

fn main() {

    println!("Hello, Welcome to Blockchain");
    let mut minner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    println!("Input miner address");
    io::stdout().flush();
    io::stdin().read_line(&mut minner_addr);
    println!("Difficulty");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("We need an integer");
    println!("Generating genesis block");

    let mut chain = blockchain::Chain::new(minner_addr.trim().to_string(),diff);

    loop{
        println!("Menu:");
        println!("1. New Transaction");
        println!("2. Mine Block", );
        println!("3. Change Difficulty");
        println!("4. Change Reward");
        println!("0. Exit");
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);

        match choice.trim().parse().unwrap(){
            0 =>{
                break;

            },
            1=>{
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                println!("Enter Sender Address");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);

                println!("Enter Receiver Address");
                io::stdout().flush();
                io::stdin().read_line(&mut receiver);
                
                println!("Enter the amount");
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                let res = chain.new_transaction(sender.trim().to_string(), receiver.trim().to_string(), amount.parse().unwrap());
                match res{
                    true => println!("transaction added"),
                    false => println!("transaction failed"),
                }

            },
            2=>{
                println!("Generating block");
                let res = chain.generate_new_block();
                match res{
                    true => println!("Block Generated successfully"),
                    false => println!("Block Generation failed")
                }
            },
            3=>{
                let mut new_diff = String::new();
                println!("Enter Difficulty");
                io::stdout().flush();
                io::stdin().read_line(&mut new_diff);
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res{
                    true => println!("Difficulty updated"),
                    false => println!("difficulty update failed"),
                }



            },
            4=>{
                let mut new_reward = String::new();
                println!("Enter new reward");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);
                let res = chain.update_reward(new_reward.trim().parse::<f32>().unwrap());
                match res{
                    true => println!("Reward updated"),
                    false => println!("reward update failed"),
                }

            },
            _=>{
                println!("Invalid choise please try again");
            }
            
        }
    }   
}
