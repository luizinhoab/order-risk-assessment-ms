CREATE TABLE assessment (
    id UUID PRIMARY KEY,
    order_number INTEGER NOT NULL,
    customer_id UUID,
    cutomer_name TEXT NOT NULL,
    customer_cpf VARCHAR(11) NOT NULL,
    card_number VARCHAR(16) NOT NULL,
    card_holder_name VARCHAR(26) NOT NULL,
    creation_date_order TIMESTAMP NOT NULL,
    status VARCHAR(15) NOT NULL,
    motivation TEXT,
    create_at TIMESTAMP NOT NULL,
    update_at TIMESTAMP NOT NULL
)