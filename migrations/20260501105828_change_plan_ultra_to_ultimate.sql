-- +goose Up
ALTER TABLE subscriptions DROP CONSTRAINT subscriptions_plan_check;

ALTER TABLE subscriptions ADD CONSTRAINT subscriptions_plan_check
    CHECK (plan IN ('basic', 'pro', 'ultimate'));

UPDATE subscriptions SET plan = 'ultimate' WHERE plan = 'ultra';

-- +goose Down
UPDATE subscriptions SET plan = 'ultra' WHERE plan = 'ultimate';

ALTER TABLE subscriptions DROP CONSTRAINT subscriptions_plan_check;

ALTER TABLE subscriptions ADD CONSTRAINT subscriptions_plan_check
    CHECK (plan IN ('basic', 'pro', 'ultra'));
