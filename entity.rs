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
    pub fn attack(&self, ennemy: Entity) {
        if (self.atk < ennemy.def) {
            println!("{self.name}: atk has no effet against {ennemy.name}")
        } else {
            // total_damage = atk * if def / 100 * atk
            continue;
        }
    }
}

fn main() {
    let blob = Entity {
        name: "bob".to_string(),
        life: 42,
        atk: 10,
        def: 20,
    };
    let wolf = Entity {
        name: "Dennis".to_string(),
        life: 42,
        def: 10,
        atk: 32,
    };
    wolf.display();
    blob.display();
}