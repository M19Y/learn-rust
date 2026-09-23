trait Jungler {
    fn retribution(&self, damage: u32) -> u32;
    fn sound(&self) {
        println!("This is default jungler sound");
    }
}

struct Hero {
    damage: u32,
    health: u8,
    armor: u8,
}

struct Player {
    hero: Hero,
    name: String,
}

impl Jungler for Player {
    fn retribution(&self, damage: u32) -> u32 {
        self.hero.damage + damage
    }
}

impl Player {
    fn stats(&self) {
        let Hero {
            damage,
            health,
            armor,
        } = self.hero;
        println!(
            "Player {}\nhealth: {}\ndamage: {}\narmor: {}",
            self.name, damage, health, armor
        );
    }
}

fn main() {
    let player1: Player = Player {
        hero: Hero {
            damage: 5,
            health: 100,
            armor: 15,
        },
        name: String::from("Not Normal"),
    };

    let current_damage: u32 = player1.hero.damage;
    let bonus_retribution: u32 = player1.retribution(10);

    println!(
        "my current damage {}, and bonus from retribution {}",
        current_damage, bonus_retribution
    );

    // we can use it again
    println!("current damage = {}", player1.hero.damage);

    player1.stats();

    // call trait default method
    player1.sound();
}
