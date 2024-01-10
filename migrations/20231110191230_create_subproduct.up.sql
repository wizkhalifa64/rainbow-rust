-- Add up migration script here
CREATE TABLE IF NOT EXISTS subproducts(
    id INTEGER NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    title VARCHAR(255) NOT NULL,
    status BOOLEAN DEFAULT FALSE,
    product_id INTEGER NOT NULL,
    alias VARCHAR(50) NOT NULL,
    FOREIGN KEY(product_id) REFERENCES products(id)
)