struct RedFox {
    enemy: bool,
    life: u32,
}

impl RedFox {
    fn get_noise(&self) -> &str {
        "Meow?"
    }
}

fn print_noise<T: Noisy>(item: T) {
    println!("{}", item.get_noise());
}

impl Noisy for u8 {
    fn get_noise(&self) -> &str {
        "BYTE!"
    }
}

fn main() {
    print_noise(5_u8);
}

trait Run {
    fn run(&self) {
        println!("I'm running!");
    }
}

struct Robot {}
impl Run for Robot {}

fn main() {
    let robot = Robot {};
    robot.run();
}

fn main() {
    struct RedFox {
        enemy: bool,
        life: u8,
    }
    let fox = RedFox {
        enemy: true,
        life: 70,
    };

    impl RedFox {
        fn new() -> Self {
            Self {
                enemy: true,
                life: 70,
            }
        }
    }

    // here the impl will create the enviroment

    let fox = RedFox::new();

    let life_left = fox.life;
    // fox.enemy = false;
    // fox.some_method();
    println!("{}", life_left);
}
