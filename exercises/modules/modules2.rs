// modules2.rs
// Make me compile! Execute `rustlings hint modules2` for hints :)


mod delicious_snacks {

    //Essas duas linhas publicam APENAS as partes dos módulos citadas
    //o resto do módulo fica indisponível
    // então nesse módulo 'delicious_snacks' ele define módulos internos 'fruits' e 'veggies'
    // esses módulos internos não ficam disponíveis por padrão
    // ao usar o 'pub use' nas primeiras linhas o código abaixo torna as partes dos sub-módulos mencionados
    // disponíveis a todos aqueles que chamarem o "módulo-pai" 'delicious_snacks' 
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}


fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
