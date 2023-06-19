use rand::Rng;
use std::io;
use std::convert::TryInto;

const TOWNHALL_LEVEL: u32 = 1;
const GOLD: u32 = 1000;
const ELIXIR: u32 = 1000;
const TROOP_CAPACITY: usize = 20;

struct Player {
    name: String,
    townhall_level: u32,
    gold: u32,
    elixir: u32,
    troop_capacity: usize,
    troops: Vec<Troop>,
}

impl Player {
    fn new(name: String) -> Player {
        Player {
            name,
            townhall_level: TOWNHALL_LEVEL,
            gold: GOLD,
            elixir: ELIXIR,
            troop_capacity: TROOP_CAPACITY,
            troops: Vec::new(),
        }
    }

    fn print_resources(&self) {
        println!("Gold: {}", self.gold);
        println!("Elixir: {}", self.elixir);
    }

    fn print_troops(&self) {
        println!("Troops:");
        for troop in &self.troops {
            println!("{}", troop);
        }
    }

    fn attack(&self) {
        let enemy_townhall_level = rand::thread_rng().gen_range(1..=10);
        if self.townhall_level > enemy_townhall_level {
            println!("You won the attack!");
        } else {
            println!("You lost the attack!");
        }
    }

    fn train_troop(&mut self, name: String, cost: u32) {
        if self.troops.len() >= self.troop_capacity {
            println!("Troop capacity reached. Upgrade your camps to train more troops.");
        } else {
            if self.gold >= cost {
                self.gold -= cost;
                let troop = Troop::new(name, cost);
                self.troops.push(troop);
            } else {
                println!("Not enough gold to train the troops.");
            }
        }
    }
}

struct Troop {
    name: String,
    cost: u32,
}

impl Troop {
    fn new(name: String, cost: u32) -> Troop {
        Troop { name, cost }
    }
}

impl std::fmt::Display for Troop {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} - Cost: {}", self.name, self.cost)
    }
}

fn game_loop() {
    println!("Welcome to Clash of Clans (CLI Version)!");
    println!("Enter Your Name: ");
    let mut player_name = String::new();
    io::stdin().read_line(&mut player_name).expect("Failed to read input");
    let player_name = player_name.trim().to_string();
    let mut player = Player::new(player_name);

    loop {
        println!("\n[MAIN MENU]");
        println!("1. Print Resources");
        println!("2. Print Troops");
        println!("3. Train Troops");
        println!("4. Attack");
        println!("5. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => player.print_resources(),
            "2" => player.print_troops(),
            "3" => {
                println!("Enter the Troop name: ");
                let mut troop_name = String::new();
                io::stdin().read_line(&mut troop_name).expect("Failed to read input");
                let troop_name = troop_name.trim().to_string();

                println!("Enter the troop cost: ");
                let mut troop_cost = String::new();
                io::stdin().read_line(&mut troop_cost).expect("Failed to read input");
                let troop_cost: u32 = troop_cost.trim().parse().unwrap();

                player.train_troop(troop_name, troop_cost);
            }
            "4" => player.attack(),
            "5" => {
                println!("Thanks for Playing Clash of Clans (CLI version)!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn main() {
    game_loop();
}
