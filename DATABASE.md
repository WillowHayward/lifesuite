Early draft of the clientside database design

# LSC
component(component_id, created, synced) PK: component_id

persona(persona_id -> component.component_id, name) PK: persona_id
tag(tag_id -> component.component_id, persona_id -> component.component_id tag_type, name, parent -> tag.tag_id) PK: tag_id
mod(mod_id -> component.component_id, changes) PK: mod_id

component_mod(component_id -> component.component_id, mod_id -> mod.mod_id) PK: component_id, mod_id

# LSJ
journal(journal_id -> component.component_id, persona_id -> persona.persona_id, name) PK: journal_id
log(log_id -> component.component_id, journal_id -> journal.journal_id, date, entry) PK: log_id

log_tag(log_id -> log.log_id, tag_id -> tag.tag_id, value) PK: log_id, tag_id

The following are for later

# LSF
ledger(ledger_id -> component.component_id, persona_id -> persona.persona_id, name) PK: ledger_id
account(account_id -> component.component_id, ledger_id -> ledger.ledger_id, name, tag_id -> tag.tag_id) PK: account_id
transaction(transaction_id -> component.component_id, debit_account -> account.account_id, credit_account -> account.account_id, amount, currency, description) PK: transaction_id

# LSL
almanac(almanac_id -> component.component_id, persona_id -> persona.persona_id, name) PK: almanac_id
list(list_id -> component.component_id, almanac_id -> almanac.almanac_id, parent -> list.list_id, heading, description, path) PK: list_id
