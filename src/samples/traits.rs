use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
struct Animal {
    name: String
}

trait Dog {
    fn bark(&self);
    fn run(&self);
}

impl Dog for Animal {
    fn bark(&self) {
       println!("{} is barking!", self.name);
    }
    fn run(&self) {
        println!("{} is running!", self.name);
        println!("Animal object: {:?}", self);
    }
}

impl fmt::Display for Animal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Animal name is: {}", self.name)
    }
}

pub fn evaluate_trait() {
    let dog = Animal {
        name: "Dog".to_string()
    };
    dog.bark();
    dog.run();

    println!("{}", dog);
}

////////////////////////////////////////////

#[derive(Debug)]
struct Monster {
    health: i32
}

#[derive(Debug)]
struct Wizard {
    health: i32
}

#[derive(Debug)]
struct Knight {
    health: i32
}

trait FightClose{}
trait FightFromDistance{}

impl FightFromDistance for Wizard {}
impl FightClose for Knight {}

fn attack_with_fireball<T: FightFromDistance + Debug>(character: &T, opponent: &mut Monster, distance: u32) {
    opponent.health -= 10;
    println!(
        "You attack with your bow. Your opponent now has {} health left.  You are now at: {:?}",
        opponent.health, character
    );
}

fn attack_with_sword<T: FightClose + Debug>(character: &T, opponent: &mut Monster) {
    opponent.health -= 10;
    println!(
        "You attack with your sword. Your opponent now has {} health left. You are now at: {:?}",
        opponent.health, character
    );
}

pub fn evaluate_trait_bounds() {
    let wizard = Wizard {
        health: 100
    };
    let knight = Knight {
        health: 200
    };
    let mut monster = Monster {
        health: 150
    };
    attack_with_fireball(&wizard, &mut monster, 300);
    attack_with_sword(&knight, &mut monster);
}
