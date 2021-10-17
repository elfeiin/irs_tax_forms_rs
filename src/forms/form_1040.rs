use super::*;

pub struct Form1040 {
	filing_status: FilingStatus,
	filer_name_and_social: (NameFIL, Social),
	spouse_name_and_social: (NameFIL, Social),
	address: Address,
	filer_pres_elec_campaign_fund: bool,
	spouse_pres_elec_campaign_fund: bool,
	deal_in_virtual_currency: bool,
	filer_can_be_dependent: bool,
	spouse_can_be_dependent: bool,
	spouse_file_separate_or_filer_dual_status: bool,
	born_64_years_before: bool,
	filer_is_blind: bool,
	spouse_born_64_years_before: bool,
	spouse_is_blind: bool,
	dependents: Vec<Dependent>,
}
