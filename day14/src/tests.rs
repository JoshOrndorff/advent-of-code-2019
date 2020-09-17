use super::*;

#[test]
fn is_all_ore_true() {
	let mut s = State::new();
	s.add("ORE".into(), 4);

	assert!(s.is_all_ore());
}

#[test]
fn is_all_ore_false() {
	let mut s = State::new();
	s.add("ORE".into(), 4);
	s.add("A".into(), 1);

	assert!(!s.is_all_ore());
}

#[test]
fn is_all_ore_empty() {
	let s = State::new();

	assert!(s.is_all_ore());
}
