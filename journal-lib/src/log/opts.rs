use uuid::Uuid;

pub struct LogOpts {
    persona: Uuid,
    journal: Uuid, // Must be journal belonging to persona

}
