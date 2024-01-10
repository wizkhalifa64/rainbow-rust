-- Add up migration script here
CREATE TABLE IF NOT EXISTS agent_subproduct(
    agent_id INTEGER REFERENCES agents(id) ON UPDATE CASCADE ON DELETE CASCADE,
    subproduct_id INTEGER REFERENCES subproducts(id) ON UPDATE CASCADE ON DELETE CASCADE
)