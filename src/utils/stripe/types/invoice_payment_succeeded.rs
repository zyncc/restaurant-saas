use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct InvoicePaymentSucceededPayload {
    pub id: String,
    pub object: String,
    pub api_version: String,
    pub created: i64,
    pub data: Data,
    pub livemode: bool,
    pub pending_webhooks: i64,
    pub request: Request,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Data {
    pub object: Object,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Object {
    pub id: String,
    pub object: String,
    pub account_country: String,
    pub account_name: String,
    pub account_tax_ids: Value,
    pub amount_due: i64,
    pub amount_overpaid: i64,
    pub amount_paid: i64,
    pub amount_remaining: i64,
    pub amount_shipping: i64,
    pub application: Value,
    pub attempt_count: i64,
    pub attempted: bool,
    pub auto_advance: bool,
    pub automatic_tax: AutomaticTax,
    pub automatically_finalizes_at: Value,
    pub billing_reason: String,
    pub collection_method: String,
    pub created: i64,
    pub currency: String,
    pub custom_fields: Value,
    pub customer: String,
    pub customer_account: Value,
    pub customer_address: CustomerAddress,
    pub customer_email: String,
    pub customer_name: String,
    pub customer_phone: Value,
    pub customer_shipping: Value,
    pub customer_tax_exempt: String,
    pub customer_tax_ids: Vec<Value>,
    pub default_payment_method: Value,
    pub default_source: Value,
    pub default_tax_rates: Vec<Value>,
    pub description: Value,
    pub discounts: Vec<Value>,
    pub due_date: Value,
    pub effective_at: i64,
    pub ending_balance: i64,
    pub footer: Value,
    pub from_invoice: Value,
    pub hosted_invoice_url: String,
    pub invoice_pdf: String,
    pub issuer: Issuer,
    pub last_finalization_error: Value,
    pub latest_revision: Value,
    pub lines: Lines,
    pub livemode: bool,
    pub metadata: Metadata2,
    pub next_payment_attempt: Value,
    pub number: String,
    pub on_behalf_of: Value,
    pub parent: Parent2,
    pub payment_settings: PaymentSettings,
    pub period_end: i64,
    pub period_start: i64,
    pub post_payment_credit_notes_amount: i64,
    pub pre_payment_credit_notes_amount: i64,
    pub receipt_number: Value,
    pub rendering: Value,
    pub shipping_cost: Value,
    pub shipping_details: Value,
    pub starting_balance: i64,
    pub statement_descriptor: Value,
    pub status: String,
    pub status_transitions: StatusTransitions,
    pub subtotal: i64,
    pub subtotal_excluding_tax: i64,
    pub test_clock: Value,
    pub total: i64,
    pub total_discount_amounts: Vec<Value>,
    pub total_excluding_tax: i64,
    pub total_pretax_credit_amounts: Vec<Value>,
    pub total_taxes: Vec<Value>,
    pub webhooks_delivered_at: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct AutomaticTax {
    pub disabled_reason: Value,
    pub enabled: bool,
    pub liability: Value,
    pub provider: Value,
    pub status: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct CustomerAddress {
    pub city: Value,
    pub country: String,
    pub line1: Value,
    pub line2: Value,
    pub postal_code: Value,
    pub state: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Issuer {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Lines {
    pub object: String,
    pub data: Vec<Daum>,
    pub has_more: bool,
    pub total_count: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Daum {
    pub id: String,
    pub object: String,
    pub amount: i64,
    pub currency: String,
    pub description: String,
    pub discount_amounts: Vec<Value>,
    pub discountable: bool,
    pub discounts: Vec<Value>,
    pub invoice: String,
    pub livemode: bool,
    pub metadata: Metadata,
    pub parent: Parent,
    pub period: Period,
    pub pretax_credit_amounts: Vec<Value>,
    pub pricing: Pricing,
    pub quantity: i64,
    pub quantity_decimal: String,
    pub subtotal: i64,
    pub taxes: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Metadata {
    pub duration: String,
    pub user_id: String,
    pub plan: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Parent {
    pub invoice_item_details: Value,
    pub subscription_item_details: SubscriptionItemDetails,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct SubscriptionItemDetails {
    pub invoice_item: Value,
    pub proration: bool,
    pub proration_details: ProrationDetails,
    pub subscription: String,
    pub subscription_item: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ProrationDetails {
    pub credited_items: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Period {
    pub end: i64,
    pub start: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Pricing {
    pub price_details: PriceDetails,
    #[serde(rename = "type")]
    pub type_field: String,
    pub unit_amount_decimal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct PriceDetails {
    pub price: String,
    pub product: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Metadata2 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Parent2 {
    pub quote_details: Value,
    pub subscription_details: SubscriptionDetails,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct SubscriptionDetails {
    pub metadata: Metadata3,
    pub subscription: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Metadata3 {
    pub duration: String,
    pub user_id: String,
    pub plan: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct PaymentSettings {
    pub default_mandate: Value,
    pub payment_method_options: PaymentMethodOptions,
    pub payment_method_types: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct PaymentMethodOptions {
    pub acss_debit: Value,
    pub bancontact: Value,
    pub card: Card,
    pub customer_balance: Value,
    pub konbini: Value,
    pub payto: Value,
    pub pix: Value,
    pub sepa_debit: Value,
    pub upi: Value,
    pub us_bank_account: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Card {
    pub request_three_d_secure: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct StatusTransitions {
    pub finalized_at: i64,
    pub marked_uncollectible_at: Value,
    pub paid_at: i64,
    pub voided_at: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Request {
    pub id: Value,
    pub idempotency_key: Value,
}
