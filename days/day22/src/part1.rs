/*
x1 = ((a * (2 ** 6)) ^ a) mod (2 ** 24)
x2 = ((x1 // (2 ** 5)) ^ x1) mod (2 ** 24)
x3 = ((x2 * (2 ** 11)) ^ x2) mod (2 ** 24)

a is initial secret, x3 is new secret

x1 = ((a << 6)  ^ a) & 0xFFFFFF
x2 = ((x1 >> 5) ^ x1) & 0xFFFFFF
x3 = ((x2 << 11) ^ x2) & 0xFFFFFF

x3 = (((((x1 >> 5) ^ x1) & 0xFFFFFF) << 11) ^ (((x1 >> 5) ^ x1) & 0xFFFFFF)) & 0xFFFFFF
x3 = ((((((((a << 6)  ^ a) & 0xFFFFFF) >> 5) ^ (((a << 6)  ^ a) & 0xFFFFFF)) & 0xFFFFFF) << 11) ^ ((((((a << 6)  ^ a) & 0xFFFFFF) >> 5) ^ (((a << 6)  ^ a) & 0xFFFFFF)) & 0xFFFFFF)) & 0xFFFFFF

m = 0xFFFFFF

x3 = ((((((((a << 6)  ^ a) & m) >> 5) ^ (((a << 6)  ^ a) & m)) & m) << 11) ^ ((((((a << 6)  ^ a) & m) >> 5) ^ (((a << 6)  ^ a) & m)) & m)) & m

X = (a << 6) ^ a
Y = X & m
Z = (Y >> 5) ^ Y
W = Z & m
R = ((W << 11) ^ W) & m

*/

const MODULO: u64 = 0xFFFFFF;

pub fn solve(_input: &str) -> u64 {
    _input.lines().map(|secret| {
        let iterations = 2000;
        let mut new_secret = secret.parse::<u64>().unwrap();
        for _ in 0..iterations {
            new_secret = ((new_secret << 6) ^ new_secret) & MODULO;
            new_secret = ((new_secret >> 5) ^ new_secret) & MODULO;
            new_secret = ((new_secret << 11) ^ new_secret) & MODULO;
        }
        new_secret
    }).sum()
}