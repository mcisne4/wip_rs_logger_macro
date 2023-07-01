use super::{CRATE_IDX_EQ, DERIVE, ENUM_DEC, EXAMPLE, LOCATION_EQ, MOD_IDX_EQ};

pub fn enum_example() -> String {
    let mut msg = EXAMPLE.to_owned();

    msg += DERIVE;
    msg += CRATE_IDX_EQ;
    msg += MOD_IDX_EQ;
    msg += LOCATION_EQ;
    msg += ENUM_DEC;
    msg += "...\n}";

    msg
}
