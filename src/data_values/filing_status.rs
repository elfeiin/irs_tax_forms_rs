pub enum FilingStatus {
	Single,
	MarriedFilingJointly,
	MarriedFilingSeparately{ name_of_spouse: String},
	HeadOfHousehold{ qualifying_child_name: String },
	QualifyingWidowOrWidower{ qualifying_child_name: String }
}