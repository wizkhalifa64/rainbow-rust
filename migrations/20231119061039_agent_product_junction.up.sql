-- Add up migration script here
CREATE TABLE IF NOT EXISTS "agent-subproduct"(
    agent_id UUID REFERENCES agents(id) ON UPDATE CASCADE ON DELETE CASCADE,
    subproduct_id INTEGER REFERENCES subproducts(id) ON UPDATE CASCADE ON DELETE CASCADE
)