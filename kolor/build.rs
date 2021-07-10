fn main() {
	if cfg!(all(feature = "f32", feature = "f64")) {
		panic!("Both f32 and f64 features enabled. Only one is allowed to be enabled at once.");
	}
}
