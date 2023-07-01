use super::{
    ENUM_DEC, ERROR_MSG_EQ, EXAMPLE, INFO_MSG_EQ, VARIANT1, VARIANT2, VARIANT3, WARN_MSG_EQ,
};

pub fn variant_example() -> String {
    let mut msg = EXAMPLE.to_owned();

    msg += ENUM_DEC;
    msg += INFO_MSG_EQ;
    msg += VARIANT1;
    msg += "  ";
    msg += WARN_MSG_EQ;
    msg += VARIANT2;
    msg += "  ";
    msg += ERROR_MSG_EQ;
    msg += VARIANT3;
    msg += "}\n";

    msg
}
