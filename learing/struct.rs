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

    let life_left = fox.lift;
    fox.enemy = false;
    fox.some_method();
}
