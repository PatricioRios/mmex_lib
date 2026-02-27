fn main() {
    #[cfg(feature = "uniffi")]
    uniffi::generate_scaffolding("src/lib.rs").expect("uniffi scaffolding generation failed");
}
