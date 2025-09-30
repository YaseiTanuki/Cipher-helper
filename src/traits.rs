pub trait Encode {
    fn encode(&self, key: i8) -> String;
}

pub trait Decode {
    fn decode(&self, key: i8) -> String;
}

pub trait BruteForce {
    fn brute_force(&self);
    fn brute_force_all(&self);
}