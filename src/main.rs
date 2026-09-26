use std::io;






fn main() {
    const LOGO:[&str;6] = [
  "██████╗ ███████╗ █████╗ ██████╗ ███████╗██████╗ ██╗   ██╗",
  "██╔══██╗██╔════╝██╔══██╗██╔══██╗██╔════╝██╔══██╗╚██╗ ██╔╝",
  "██████╔╝█████╗  ███████║██████╔╝█████╗  ██████╔╝ ╚████╔╝ ",
  "██╔══██╗██╔══╝  ██╔══██║██╔═══╝ ██╔══╝  ██╔══██╗  ╚██╔╝  ",
  "██║  ██║███████╗██║  ██║██║     ███████╗██║  ██║   ██║   ",
  "╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝     ╚══════╝╚═╝  ╚═╝   ╚═╝   ",
    ];

    println!("{}",LOGO.join("\n"));

    let mut user_choice = String::new();

    #[derive(Debug)]
    enum Choice {
        Transaction,
        TrackWallet,
        ContractData,
        Exit,
        Other
    }

    


    let choice:[&str;4] = [
        "transaction",
        "get wallet data",
        "contract data",
        "exit"
    ];

    println!("Choose one option and write its number down:");
    for (index,cho)    in choice.iter().enumerate() {
        println!("{}.{}",index+1,cho);
    }
    println!("your number:");

    
    io::stdin().read_line(&mut user_choice).unwrap();

   
       let remember =  match user_choice.as_str().trim() {
            "1" => Choice::Transaction,
            "2" => Choice::TrackWallet,
            "3" => Choice::ContractData,
            "4" => Choice::Exit,
            _ => Choice::Other
        };
                             
            
        match remember {
            Choice::Transaction =>transaction(),
            Choice::TrackWallet => get_wallet_data(),
            Choice::ContractData => contract_data(),
            Choice::Exit => println!("asd"),
            Choice::Other => println!("try again") 
        }
            



}

fn get_wallet_data() {
    println!("you chose track wallet function");
}
fn transaction() {
    println!("you chose transaction function");
}
fn contract_data() {
    println!("you chose contract data function");
}