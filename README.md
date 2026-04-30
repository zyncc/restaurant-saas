# 🍽️ MenuFlow

A multi-tenant restaurant menu SaaS that lets restaurant owners create and manage their digital menus, handle table sessions via QR codes, and accept orders from customers — all through a modern web interface.

---

##  It Does

Restaurant owners subscribe to a plan, create their restaurant listing, set up their menu and tables, and invite staff. Customers scan a QR code shown by staff on their device, enter their name, and start ordering — no app download, no OTP, no friction.

---

## How It Works

### For Restaurant Owners

```
Sign Up → Choose Plan (Stripe) → Create Restaurant → Add Menu & Tables → Invite Staff
```

Owners manage everything from an admin dashboard: menu categories, menu items, staff roles, table setup, and live order tracking.

### For Staff

Staff log in with email and password. From the table management page they can:

- View a live QR code for each table
- Refresh a table session when a new party arrives (old QR instantly invalidated)
- Monitor incoming orders and update their status

### For Customers

```
Scan QR shown by staff → Enter name (phone/email optional) → Browse menu → Place order
```

No app, no account, no OTP. The QR token proves physical presence. Once the table session is refreshed by staff, that QR is dead.

---

## Subscription Plans

| Plan    | Description                       |
| ------- | --------------------------------- |
| `basic` | Entry level for small restaurants |
| `pro`   | Mid-tier with extended features   |
| `ultra` | Full feature access               |

Billing is handled entirely by Stripe. Webhook events drive subscription state — never rely on redirect alone.

---

## Tech Stack

> Update this section with your actual choices

| Layer               | Technology                   |
| ------------------- | ---------------------------- |
| Language            | Go                           |
| Database            | PostgreSQL                   |
| Migrations          | Goose                        |
| Payments            | Stripe                       |
| Auth (owners/staff) | Email + Password (JWT)       |
| Auth (customers)    | QR table session token (JWT) |
| File Storage        | TBD                          |
| Hosting             | TBD                          |

---

## Database Schema

### Core Tables

| Table              | Description                                                        |
| ------------------ | ------------------------------------------------------------------ |
| `owners`           | SaaS subscribers — billing identity, plan status, onboarding state |
| `subscriptions`    | Stripe subscription details, one per owner                         |
| `restaurants`      | Restaurant listings, each owned by one owner                       |
| `restaurant_staff` | Staff members scoped to a restaurant, no billing relation          |
| `tables`           | Physical tables inside a restaurant                                |
| `table_sessions`   | Active QR session per table, deleted and recreated on reset        |
| `menu_categories`  | Groupings like Starters, Mains, Drinks                             |
| `menu_items`       | Individual dishes with price, availability, dietary flags          |
| `orders`           | Customer orders tied to a table session                            |
| `order_items`      | Line items snapshot — name and price captured at order time        |
| `audit_logs`       | Immutable log of every staff action with before/after state        |

### Auth Separation

There are two completely independent auth systems:

```
owners + restaurant_staff  →  Email/Password  →  JWT { type: "staff" }
customers                  →  QR scan token   →  JWT { type: "customer" }
```

A customer JWT can never access a staff route and vice versa — the `type` field is checked at the middleware level.

### Table Session Flow

```
Staff opens table management page
  → QR code displayed on screen (encodes session token in URL)
  → Customer scans → token validated → session JWT issued → menu unlocked
  → Customer orders food
  → Party leaves → staff hits "Reset Table"
  → DELETE old session + INSERT new session (in a transaction)
  → Old QR is immediately dead, new QR is live
```

The QR URL never changes — only the token it points to does, server-side.

---

## Onboarding Flow

Owners go through a 3-step funnel tracked via `onboarding_step` on the `owners` table:

| Step | Value               | What Happens                                            |
| ---- | ------------------- | ------------------------------------------------------- |
| 1    | `subscription`      | Owner signed up but hasn't paid yet                     |
| 2    | `create_restaurant` | Stripe payment confirmed, redirect to create restaurant |
| 3    | `complete`          | Fully onboarded, redirect to dashboard                  |

On every login, `onboarding_step` is checked and the user is routed to the correct step if they haven't completed it yet.

---

## Stripe Webhooks

The app handles the following Stripe events:

| Event                           | Action                                                         |
| ------------------------------- | -------------------------------------------------------------- |
| `checkout.session.completed`    | Upsert subscription, set `onboarding_step = create_restaurant` |
| `customer.subscription.updated` | Update plan, status, period dates                              |
| `customer.subscription.deleted` | Set status to `cancelled`, update `ended_at`                   |
| `invoice.payment_failed`        | Set status to `past_due`, warn owner                           |

Subscription rows are upserted on every event using `ON CONFLICT (owner_id) DO UPDATE` — one row per owner, always up to date.

---

## Order Status Lifecycle

```
pending → confirmed → preparing → ready → served
                                        ↘ cancelled
```

---

## Audit Logging

Every staff action is recorded in `audit_logs` with:

- Who did it (staff name + role snapshot)
- What they did (`order.status_updated`, `menu_item.price_updated`, etc.)
- Which entity was affected
- Full before/after state as JSONB

Logs are immutable — no `updated_at`, never modified after insert.

---

## Running Migrations

```bash
# Apply all pending migrations
goose up

# Roll back the last migration
goose down

# Check current migration status
goose status
```

---

## Project Structure

> Update as the project grows

```
.
├── cmd/                  # Entry points
├── internal/
│   ├── handlers/         # HTTP handlers
│   ├── middleware/        # Auth, logging
│   ├── db/               # DB queries
│   └── stripe/           # Webhook handlers
├── migrations/           # Goose SQL migrations
└── README.md
```

---

## Environment Variables

```env
DATABASE_URL=
JWT_SECRET=
STRIPE_SECRET_KEY=
STRIPE_WEBHOOK_SECRET=
```
