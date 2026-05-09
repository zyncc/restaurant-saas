-- +goose Up
-- +goose StatementBegin
CREATE TABLE
    restaurants (
        id UUID PRIMARY KEY,
        name TEXT NOT NULL,
        slug TEXT UNIQUE NOT NULL,
        description TEXT NOT NULL,
        logo_url TEXT,
        address TEXT NOT NULL,
        phone TEXT NOT NULL,
        is_active BOOLEAN NOT NULL DEFAULT true,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT now ()
    );

CREATE TABLE
    restaurant_staff (
        id UUID PRIMARY KEY,
        stripe_customer_id TEXT UNIQUE,
        restaurant_id UUID REFERENCES restaurants (id) ON DELETE CASCADE,
        name TEXT NOT NULL,
        email TEXT UNIQUE NOT NULL,
        password_hash TEXT NOT NULL,
        role TEXT NOT NULL CHECK (role IN ('owner', 'manager', 'staff', 'user')),
        onboarding_step TEXT CHECK (
            onboarding_step IN ('subscription', 'create_restaurant', 'complete')
        ),
        is_active BOOLEAN NOT NULL DEFAULT true,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT now ()
    );

CREATE TABLE
    customers (
        id UUID PRIMARY KEY,
        name TEXT NOT NULL,
        phone TEXT NOT NULL,
        email TEXT NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now ()
    );

CREATE TABLE
    subscriptions (
        id UUID PRIMARY KEY,
        staff_id UUID NOT NULL UNIQUE REFERENCES restaurant_staff (id) ON DELETE CASCADE,
        stripe_subscription_id TEXT UNIQUE NOT NULL,
        stripe_customer_id TEXT NOT NULL,
        stripe_price_id TEXT NOT NULL,
        plan TEXT NOT NULL CHECK (plan IN ('basic', 'pro', 'ultimate')),
        duration TEXT NOT NULL CHECK (duration in ('1-month', '1-year')),
        status TEXT NOT NULL CHECK (
            status IN ('active', 'trialing', 'past_due', 'cancelled')
        ),
        current_period_start TIMESTAMPTZ NOT NULL,
        current_period_end TIMESTAMPTZ NOT NULL,
        cancel_at TIMESTAMPTZ,
        cancelled_at TIMESTAMPTZ,
        ended_at TIMESTAMPTZ,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT now ()
    );

CREATE TABLE
    tables (
        id UUID PRIMARY KEY,
        restaurant_id UUID NOT NULL REFERENCES restaurants (id) ON DELETE CASCADE,
        table_number TEXT NOT NULL,
        label TEXT,
        is_active BOOLEAN NOT NULL DEFAULT true,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
        UNIQUE (restaurant_id, table_number)
    );

CREATE TABLE
    staff_sessions (
        id UUID PRIMARY KEY,
        staff_id UUID NOT NULL REFERENCES restaurant_staff (id) ON DELETE CASCADE,
        session_token TEXT UNIQUE NOT NULL,
        ip_address TEXT,
        user_agent TEXT,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
        expires_at TIMESTAMPTZ NOT NULL DEFAULT now () + INTERVAL '24 hours'
    );

CREATE TABLE
    table_sessions (
        id UUID PRIMARY KEY,
        table_id UUID UNIQUE NOT NULL REFERENCES tables (id) ON DELETE CASCADE,
        restaurant_id UUID NOT NULL REFERENCES restaurants (id) ON DELETE CASCADE,
        table_token TEXT UNIQUE NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now ()
    );

CREATE TABLE
    menu_categories (
        id UUID PRIMARY KEY,
        restaurant_id UUID NOT NULL REFERENCES restaurants (id) ON DELETE CASCADE,
        name TEXT NOT NULL,
        description TEXT NOT NULL,
        sort_order INTEGER NOT NULL DEFAULT 0,
        is_active BOOLEAN NOT NULL DEFAULT true,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now ()
    );

CREATE TABLE
    menu_items (
        id UUID PRIMARY KEY,
        restaurant_id UUID NOT NULL REFERENCES restaurants (id) ON DELETE CASCADE,
        category_id UUID REFERENCES menu_categories (id) ON DELETE SET NULL,
        name TEXT NOT NULL,
        description TEXT,
        price NUMERIC(10, 2) NOT NULL,
        image_url TEXT NOT NULL,
        is_available BOOLEAN NOT NULL DEFAULT true,
        food_type TEXT NOT NULL CHECK (food_type IN ('veg', 'nonveg', 'egg')),
        sort_order INTEGER NOT NULL DEFAULT 0,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT now ()
    );

CREATE TABLE
    orders (
        id UUID PRIMARY KEY,
        restaurant_id UUID NOT NULL REFERENCES restaurants (id) ON DELETE CASCADE,
        table_session_id UUID NOT NULL REFERENCES table_sessions (id) ON DELETE RESTRICT,
        customer_name TEXT,
        customer_phone TEXT,
        customer_email TEXT,
        status TEXT NOT NULL DEFAULT 'pending' CHECK (
            status IN (
                'pending',
                'confirmed',
                'preparing',
                'ready',
                'served',
                'cancelled'
            )
        ),
        total_amount NUMERIC(10, 2),
        notes TEXT,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT now ()
    );

CREATE TABLE
    order_items (
        id UUID PRIMARY KEY,
        order_id UUID NOT NULL REFERENCES orders (id) ON DELETE CASCADE,
        menu_item_id UUID REFERENCES menu_items (id) ON DELETE SET NULL,
        name TEXT NOT NULL,
        price NUMERIC(10, 2) NOT NULL,
        quantity INTEGER NOT NULL CHECK (quantity > 0),
        created_at TIMESTAMPTZ NOT NULL DEFAULT now ()
    );

CREATE TABLE
    audit_logs (
        id UUID PRIMARY KEY,
        restaurant_id UUID NOT NULL REFERENCES restaurants (id) ON DELETE CASCADE,
        staff_id UUID NOT NULL REFERENCES restaurant_staff (id) ON DELETE SET NULL,
        staff_name TEXT NOT NULL,
        staff_role TEXT NOT NULL,
        action TEXT NOT NULL, -- e.g. 'order.deleted'
        entity TEXT NOT NULL, -- e.g. 'order', 'menu_item', 'table_session'
        created_at TIMESTAMPTZ NOT NULL DEFAULT now ()
    );

-- indexes
CREATE INDEX idx_restaurant_staff_restaurant ON restaurant_staff (restaurant_id);

CREATE INDEX idx_tables_restaurant ON tables (restaurant_id);

CREATE INDEX idx_table_sessions_table ON table_sessions (table_id);

CREATE INDEX idx_table_sessions_restaurant ON table_sessions (restaurant_id);

CREATE INDEX idx_menu_items_restaurant ON menu_items (restaurant_id);

CREATE INDEX idx_menu_items_category ON menu_items (category_id);

CREATE INDEX idx_orders_restaurant ON orders (restaurant_id);

CREATE INDEX idx_orders_table_session ON orders (table_session_id);

CREATE INDEX idx_order_items_order ON order_items (order_id);

CREATE INDEX idx_audit_logs_restaurant ON audit_logs (restaurant_id);

CREATE INDEX idx_audit_logs_staff ON audit_logs (staff_id);

CREATE INDEX idx_audit_logs_created_at ON audit_logs (created_at DESC);

-- +goose StatementEnd
-- +goose Down
-- +goose StatementBegin
DROP INDEX IF EXISTS idx_audit_logs_created_at;
DROP INDEX IF EXISTS idx_audit_logs_staff;
DROP INDEX IF EXISTS idx_audit_logs_restaurant;
DROP INDEX IF EXISTS idx_order_items_order;
DROP INDEX IF EXISTS idx_orders_table_session;
DROP INDEX IF EXISTS idx_orders_restaurant;
DROP INDEX IF EXISTS idx_menu_items_category;
DROP INDEX IF EXISTS idx_menu_items_restaurant;
DROP INDEX IF EXISTS idx_table_sessions_restaurant;
DROP INDEX IF EXISTS idx_table_sessions_table;
DROP INDEX IF EXISTS idx_tables_restaurant;
DROP INDEX IF EXISTS idx_restaurant_staff_restaurant;

DROP TABLE IF EXISTS audit_logs;

DROP TABLE IF EXISTS order_items;

DROP TABLE IF EXISTS orders;

DROP TABLE IF EXISTS menu_items;

DROP TABLE IF EXISTS menu_categories;

DROP TABLE IF EXISTS table_sessions;

DROP TABLE IF EXISTS staff_sessions;

DROP TABLE IF EXISTS tables;

DROP TABLE IF EXISTS subscriptions;

DROP TABLE IF EXISTS customers;

DROP TABLE IF EXISTS restaurant_staff;

DROP TABLE IF EXISTS restaurants;
-- +goose StatementEnd
