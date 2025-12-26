use macro_rules_attribute::derive_alias;

derive_alias! {
	// Basic compare (no float)
	#[derive(Cmp!)] = #[derive(PartialEq, Eq, PartialOrd, Ord)];

	// Basic hash (e.g., for hash keys)
	#[derive(Hash!)] = #[derive(PartialEq, Eq, Hash)];

	// Standard Id alias
	#[derive(Id!)] = #[derive(
		Debug,
		Clone,
		crate::Hash!,
		derive_more::Display,
		derive_more::Deref,
		derive_more::From
	)];
}
