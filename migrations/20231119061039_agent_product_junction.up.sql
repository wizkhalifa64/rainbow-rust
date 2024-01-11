-- Add up migration script here
CREATE TABLE IF NOT EXISTS tbl_agent_subproduct(
    agent_id INTEGER REFERENCES tbl_agents(id) ON UPDATE CASCADE ON DELETE CASCADE,
    subproduct_id INTEGER REFERENCES tbl_subproducts(id) ON UPDATE CASCADE ON DELETE CASCADE
);