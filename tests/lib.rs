#![no_std]
#[macro_use]
extern crate digest;
extern crate sha2ni;

use digest::dev::{one_million_a, digest_test};

new_test!(sha224_main, "sha224", sha2ni::Sha224, digest_test);
new_test!(sha256_main, "sha256", sha2ni::Sha256, digest_test);
new_test!(sha384_main, "sha384", sha2ni::Sha384, digest_test);
new_test!(sha512_main, "sha512", sha2ni::Sha512, digest_test);
new_test!(sha512_224_main, "sha512_224", sha2ni::Sha512Trunc224, digest_test);
new_test!(sha512_256_main, "sha512_256", sha2ni::Sha512Trunc256, digest_test);

#[test]
fn sha256_1million_a() {
    let output = include_bytes!("data/sha256_one_million_a.bin");
    one_million_a::<sha2ni::Sha256>(output);
}

#[test]
fn sha512_1million_a() {
    let output = include_bytes!("data/sha512_one_million_a.bin");
    one_million_a::<sha2ni::Sha512>(output);
}
