-- +goose Up
-- +goose StatementBegin
ALTER TABLE restaurant_staff ADD COLUMN stripe_customer_id TEXT UNIQUE;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE restaurant_staff DROP COLUMN stripe_customer_id;
-- +goose StatementEnd
