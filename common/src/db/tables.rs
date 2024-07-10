use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum DbTables {
    Component,
    Persona,
    Tag,
    Mod,
    ComponentMod,
    Journal,
    Log,
    LogTag,
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
            DbTables::LogTag => "log_tag",
        }
    }

    pub fn dependencies(&self) -> HashSet<DbTables> {
        let mut deps = HashSet::new();
        match self {
            DbTables::Component => (),
            DbTables::Persona => {
                deps.insert(DbTables::Component);
            }
            DbTables::Tag => {
                deps.insert(DbTables::Component);
            }
            DbTables::Mod => {
                deps.insert(DbTables::Component);
            }
            DbTables::ComponentMod => {
                deps.insert(DbTables::Component);
                deps.insert(DbTables::Mod);
            }
            DbTables::Journal => {
                deps.insert(DbTables::Component);
                deps.insert(DbTables::Persona);
                deps.insert(DbTables::Mod);
            }
            DbTables::Log => {
                deps.insert(DbTables::Journal);
            }
            DbTables::LogTag => {
                deps.insert(DbTables::Log);
                deps.insert(DbTables::Tag);
            }
        }

        add_subdeps(deps)
    }
}

fn add_subdeps(shallow_deps: HashSet<DbTables>) -> HashSet<DbTables> {
    let mut checked: HashSet<DbTables> = HashSet::new();
    let mut unchecked = shallow_deps.clone();
    while !unchecked.is_empty() {
        let target = unchecked.iter().next().unwrap().clone();
        unchecked.remove(&target);
        checked.insert(target.clone());
        let sub = target.dependencies();
        for d in sub.iter() {
            if !checked.contains(d) {
                unchecked.insert(d.clone());
            }
        }
    }

    checked
}
