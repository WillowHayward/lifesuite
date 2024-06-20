pub enum DbTables {
    Component,
    Persona,
    Tag,
    Mod,
    ComponentMod,
    Journal,
    Log,
    LogTag
}

impl DbTables {
    pub fn table_name(&self) -> &'static str {
        match self {
            DbTables::Component => "component",
            DbTables::Persona => "persona",
            DbTables::Tag => "tag",
            DbTables::Mod => "mod",
            DbTables::ComponentMod => "component_mod",
            DbTables::Journal => "journal",
            DbTables::Log => "log",
            DbTables::LogTag => "log_tag"
        }
    }
}
