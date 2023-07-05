pub enum EnumAttrs {
    CrateIdx,
    ModuleIdx,
    LogPath,
}
impl EnumAttrs {
    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            Self::CrateIdx => "crate_idx",
            Self::ModuleIdx => "module_idx",
            Self::LogPath => "log_path",
        }
    }
}

pub enum VariantAttrs {
    InfoMsg,
    WarnMsg,
    ErrorMsg,
}
impl VariantAttrs {
    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            Self::InfoMsg => "info_msg",
            Self::WarnMsg => "warn_msg",
            Self::ErrorMsg => "error_msg",
        }
    }
}

pub enum Attrs {
    CrateIdx,
    ModuleIdx,
    LogPath,
    InfoMsg,
    WarnMsg,
    ErrorMsg,
}
impl Attrs {
    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            Self::CrateIdx => EnumAttrs::CrateIdx.as_str(),
            Self::ModuleIdx => EnumAttrs::ModuleIdx.as_str(),
            Self::LogPath => EnumAttrs::LogPath.as_str(),
            Self::InfoMsg => VariantAttrs::InfoMsg.as_str(),
            Self::WarnMsg => VariantAttrs::WarnMsg.as_str(),
            Self::ErrorMsg => VariantAttrs::ErrorMsg.as_str(),
        }
    }
}
