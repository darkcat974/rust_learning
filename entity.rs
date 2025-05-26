struct Entity {
    name: String,
    life: i64,
    atk: i64,
    def: i64
}

impl Entity {
    pub fn display(&self) {
        println!(
            "{}: ({}hp, {}def, {}atk)",
             self.name, self.life, self.def, self.atk
        );
    }
    pub fn attack(&self,  ennemy: &mut Entity) {
        if self.atk < ennemy.def {
            println!("{}: atk has no effet against {}", self.name, ennemy.name);
        } else {
            let total_damage = self.atk - ennemy.def;
            println!("{}: dealt {} to {}", self.name, total_damage, ennemy.name);
            ennemy.life -= total_damage
        }
    }
}

fn main() {
    let mut blob = Entity {
        name: "bob".to_string(),
        life: 42,
        atk: 10,
        def: 20,
    };
    let mut wolf = Entity {
        name: "Dennis".to_string(),
        life: 42,
        def: 10,
        atk: 32,
    };
    wolf.display();
    blob.display();
    wolf.attack(&mut blob);
    blob.attack(&mut wolf);
    wolf.display();
    blob.display();
}
