-- Shop-Proof Database Schema
-- This schema enforces all invariants at the database level

PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;
PRAGMA synchronous = NORMAL;

-- Products table
-- Invariant: price_cents >= 0
CREATE TABLE IF NOT EXISTS products (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    price_cents INTEGER NOT NULL CHECK (price_cents >= 0)
) STRICT;

-- Stock table
-- Invariant: qty >= 0
-- Foreign key to products
CREATE TABLE IF NOT EXISTS stock (
    product_id INTEGER PRIMARY KEY REFERENCES products(id),
    qty INTEGER NOT NULL CHECK (qty >= 0)
) STRICT;

-- Orders table
-- Invariants:
-- 1. subtotal_cents >= 0
-- 2. discount_cents >= 0
-- 3. tax_cents >= 0
-- 4. total_cents >= 0
-- 5. discount_cents <= subtotal_cents
-- 6. total_cents = subtotal_cents - discount_cents + tax_cents
CREATE TABLE IF NOT EXISTS orders (
    id INTEGER PRIMARY KEY,
    subtotal_cents INTEGER NOT NULL CHECK (subtotal_cents >= 0),
    discount_cents INTEGER NOT NULL CHECK (discount_cents >= 0),
    tax_cents INTEGER NOT NULL CHECK (tax_cents >= 0),
    total_cents INTEGER NOT NULL CHECK (total_cents >= 0),
    created_at INTEGER NOT NULL,
    CHECK (discount_cents <= subtotal_cents),
    CHECK (total_cents = subtotal_cents - discount_cents + tax_cents)
) STRICT;

-- Order lines table
-- Invariants:
-- 1. qty > 0
-- 2. unit_price_cents >= 0
-- Foreign keys to orders and products
CREATE TABLE IF NOT EXISTS order_lines (
    order_id INTEGER NOT NULL REFERENCES orders(id),
    product_id INTEGER NOT NULL REFERENCES products(id),
    qty INTEGER NOT NULL CHECK (qty > 0),
    unit_price_cents INTEGER NOT NULL CHECK (unit_price_cents >= 0),
    PRIMARY KEY (order_id, product_id)
) STRICT;

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_products_id ON products(id);
CREATE INDEX IF NOT EXISTS idx_stock_product_id ON stock(product_id);
CREATE INDEX IF NOT EXISTS idx_orders_id ON orders(id);
CREATE INDEX IF NOT EXISTS idx_orders_created_at ON orders(created_at);
CREATE INDEX IF NOT EXISTS idx_order_lines_order_id ON order_lines(order_id);
CREATE INDEX IF NOT EXISTS idx_order_lines_product_id ON order_lines(product_id);

-- Trigger to update subtotal when order lines are inserted/updated/deleted
-- This ensures that subtotal always equals sum of line subtotals
-- Note: This is handled at the application level, not in the database
-- The database only enforces the CHECK constraints

-- View for available stock (quantity in stock minus reserved in orders)
-- This is for reporting purposes only
CREATE VIEW IF NOT EXISTS available_stock AS
SELECT 
    s.product_id,
    s.qty - COALESCE(SUM(ol.qty), 0) as available_qty
FROM stock s
LEFT JOIN order_lines ol ON s.product_id = ol.product_id
GROUP BY s.product_id, s.qty;
