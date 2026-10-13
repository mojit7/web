# Shop-Proof 🏛️

**فروشگاه آنلاین کاملا اثبات‌پذیر برای 1000 کاربر و 1000 کالا**

## 🎯 فلسفه پروژه

Shop-Proof یک سیستم فروشگاهی است که **به طور ریاضی اثبات شده** است. هر خط کد دامنه قبل از پیاده‌سازی در Lean 4 اثبات می‌شود.

**شعار:** کد اجرایی = کد اثبات‌شده

## 📋 ویژگی‌های کلیدی

✅ **اثبات فرمال:** همه قوانین کسب‌وکار در Lean 4 اثبات شده‌اند  
✅ **بدون float برای پول:** پول همیشه به صورت integer (cents) ذخیره می‌شود  
✅ **Invariant‌های runtime:** همه تراکنش‌ها invariant‌ها را چک می‌کنند  
✅ **دیتابیس امن:** SQLite با WAL + STRICT + CHECK constraints  
✅ **همزمانی اثبات‌شده:** TLA+ برای اثبات عدم race condition  
✅ **تست‌های property-based:** proptest برای هزاران سناریوی رندوم  
✅ **سادگی رادیکال:** هیچ feature اضافی، فقط چی که لازم است  

## 🏗️ ساختار پروژه

```
shop-proof/
├── lean/                       # منبع حقیقت (Lean 4)
│   ├── Shop/
│   │   ├── Money.lean         # اثبات‌های پول
│   │   ├── Rate.lean          # اثبات‌های نرخ
│   │   ├── Tax.lean           # اثبات‌های مالیات
│   │   ├── Discount.lean      # اثبات‌های تخفیف
│   │   ├── Coupon.lean        # اثبات‌های کوپن
│   │   ├── Product.lean       # تعاریف محصول
│   │   ├── Stock.lean         # اثبات‌های موجودی
│   │   ├── Order.lean         # اثبات‌های سفارش
│   │   ├── Payment.lean       # تعاریف پرداخت
│   │   └── Invariants.lean    # اثبات‌های invariant
│   ├── lakefile.lean
│   └── lean-toolchain
│
├── tla/                        # اثبات همزمانی
│   ├── Order.tla
│   ├── Stock.tla
│   └── Order.cfg
│
├── core/                       # Rust (استخراج از Lean)
│   ├── Cargo.toml
│   └── src/
│       ├── domain/             # انواع دامنه
│       │   ├── money.rs
│       │   ├── rate.rs
│       │   ├── tax.rs
│       │   ├── discount.rs
│       │   ├── product.rs
│       │   ├── stock.rs
│       │   ├── order.rs
│       │   └── payment.rs
│       │
│       └── invariants/         # چک‌های runtime
│           ├── mod.rs
│           ├── money.rs
│           ├── order.rs
│           ├── stock.rs
│           ├── discount.rs
│           └── tax.rs
│
├── web/                        # لایه وب
│   ├── Cargo.toml
│   └── src/
│       ├── routes/            # مسیرها
│       │   ├── mod.rs
│       │   ├── index.rs
│       │   ├── products.rs
│       │   ├── cart.rs
│       │   ├── orders.rs
│       │   └── admin.rs
│       │
│       └── main.rs
│
├── migrations/                 # Migration‌های دیتابیس
│   └── 001_init.sql
│
├── tests/                      # تست‌ها
│   ├── property/
│   └── integration/
│
├── Caddyfile                  # تنظیمات Caddy
├── shop-proof.service         # systemd service
├── litestream.yml            # تنظیمات بکاپ
└── README.md
```

## 📦 استک تکنولوژی

| لایه | تکنولوژی | نسخه | منبع |
|------|----------|-------|--------|
| زبان دامنه | Rust | stable | [rust-lang.org](https://rust-lang.org) |
| زبان اثبات | Lean 4 | آخرین stable | [lean-lang.org](https://lean-lang.org) |
| همزمانی | TLA+ | TLC | [lamport.azurewebsites.net](https://lamport.azurewebsites.net/tla) |
| Property test | proptest | آخرین | [crates.io/proptest](https://crates.io/crates/proptest) |
| دیتابیس | SQLite | WAL + STRICT | [sqlite.org](https://sqlite.org) |
| Backup | litestream | آخرین | [litestream.io](https://litestream.io) |
| HTTP | axum | آخرین | [github.com/tokio-rs/axum](https://github.com/tokio-rs/axum) |
| فرانت‌اند | HTMX | آخرین | [htmx.org](https://htmx.org) |
| CSS | Pico.css | آخرین | [picocss.com](https://picocss.com) |
| Reverse proxy | Caddy | v2 | [caddyserver.com](https://caddyserver.com) |
| Deploy | systemd | استاندارد | [freedesktop.org](https://freedesktop.org) |

## 🚀 شروع سریع

### پیش‌نیازها

- [Rust 1.75+](https://rust-lang.org)
- [Lean 4](https://lean-lang.org)
- [SQLite](https://sqlite.org)
- [JavaScript](https://nodejs.org) (برای HTMX)

### نصب و راه‌اندازی

```bash
# کلون کردن مخزن
git clone git@github.com:mojit7/web.git shop-proof
cd shop-proof

# ساخت پروژه Lean
cd lean
lake build

# ساخت پروژه Core
cd ../core
cargo build --release

# ساخت پروژه Web
cd ../web
cargo build --release

# اجرا
./target/release/shop-proof-web
```

### اجرا با Caddy

```bash
# کپی کردن باینری
cp target/release/shop-proof-web /usr/local/bin/shop-proof

# کپی کردن فایل‌های تنظیمات
cp Caddyfile /etc/caddy/Caddyfile
cp shop-proof.service /etc/systemd/system/
cp litestream.yml /etc/litestream.yml

# راه‌اندازی systemd
systemctl daemon-reload
systemctl enable shop-proof
systemctl start shop-proof

# راه‌اندازی Caddy
systemctl restart caddy
```

## 📊 مسیرهای API

| Method | Path | توضیح |
|--------|------|---------|
| GET | `/` | صفحه اصلی |
| GET | `/products` | لیست محصولات |
| GET | `/product/:id` | جزئیات محصول |
| POST | `/cart/add` | افزودن به سبد |
| GET | `/cart` | مشاهده سبد |
| POST | `/cart/remove` | حذف از سبد |
| POST | `/order` | ثبت سفارش |
| GET | `/order/:id` | مشاهده سفارش |
| GET | `/admin/login` | ورود ادمین |
| GET | `/admin/products` | مدیریت محصولات |
| POST | `/admin/products` | افزودن محصول |
| POST | `/admin/stock` | تنظیم موجودی |

## 🔐 Invariant‌های اصلی

### پول (Money)
1. ✅ پول هرگز منفی نمی‌شود
2. ✅ جمع پول associative است
3. ✅ جمع پول commutative است
4. ✅ تفریق پول از خودش صفر می‌شود
5. ✅ ضرب پول روی جمع توزیع می‌شود

### سفارش (Order)
1. ✅ total = subtotal - discount + tax
2. ✅ subtotal = sum of line subtotals
3. ✅ tax = computeTax (subtotal - discount) taxRate

### موجودی (Stock)
1. ✅ موجودی هرگز منفی نمی‌شود
2. ✅ کاهش موجودی امن است (n ≤ qty → (decrement n).qty ≥ 0)

### تخفیف (Discount)
1. ✅ تخفیف منفی نیست
2. ✅ تخفیف از subtotal بیشتر نیست
3. ✅ تخفیف صفر، مبلغ را تغییر نمی‌دهد

### مالیات (Tax)
1. ✅ مالیات منفی نیست (اگر base غیرمنفی باشد)
2. ✅ مالیات با نرخ 0، صفر است

## 📝 اثبات‌های Lean 4

### Money.lean
```lean
theorem money_add_assoc (a b c : Money) : (a + b) + c = a + (b + c)
theorem money_add_comm (a b : Money) : a + b = b + a
theorem money_sub_self (a : Money) : a - a = 0
theorem money_mul_distrib (a b : Money) (n : Nat) : (a + b) * n = a*n + b*n
```

### Tax.lean
```lean
theorem tax_non_negative (base : Money) (rate : Rate) : base ≥ 0 → computeTax base rate ≥ 0
theorem tax_zero_rate (base : Money) : computeTax base 0 = 0
```

### Discount.lean
```lean
theorem discount_non_negative (d : Discount) : d.amount ≥ 0
theorem discount_bounded (d : Discount) (sub : Money) : d.amount ≤ sub
theorem discount_zero (sub : Money) : applyDiscount sub zeroDiscount = sub
```

### Order.lean
```lean
theorem order_total_formula (o : Order) : o.total = o.subtotal - o.discount + o.tax
theorem order_subtotal_sum (o : Order) : o.subtotal = (o.lines.map Line.subtotal).sum
theorem order_tax_on_discounted (o : Order) : o.tax = computeTax (o.subtotal - o.discount) o.taxRate
```

### Stock.lean
```lean
theorem stock_non_negative (s : Stock) : s.qty ≥ 0
theorem stock_decrement_safe (s : Stock) (n : Nat) : n ≤ s.qty → (s.decrement n).qty ≥ 0
```

### Invariants.lean
```lean
theorem order_preserves_invariants (o : Order) (s : Stock) :
  validOrder o → validStock s → sufficientStock o s →
  validOrder (executeOrder o s).1 ∧ validStock (executeOrder o s).2
```

## 🔄 TLA+ Specifications

### Order.tla
- مشخصات تراکنش سفارش
- اثبات serializability
- جلوگیری از race در کاهش موجودی

### Stock.tla
- مشخصات کاهش موجودی
- اثبات عدم منفی شدن
- اثبات atomicity

## 🗃️ Schema دیتابیس

```sql
-- Products
CREATE TABLE products (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    price_cents INTEGER NOT NULL CHECK (price_cents >= 0)
) STRICT;

-- Stock
CREATE TABLE stock (
    product_id INTEGER PRIMARY KEY REFERENCES products(id),
    qty INTEGER NOT NULL CHECK (qty >= 0)
) STRICT;

-- Orders
CREATE TABLE orders (
    id INTEGER PRIMARY KEY,
    subtotal_cents INTEGER NOT NULL CHECK (subtotal_cents >= 0),
    discount_cents INTEGER NOT NULL CHECK (discount_cents >= 0),
    tax_cents INTEGER NOT NULL CHECK (tax_cents >= 0),
    total_cents INTEGER NOT NULL CHECK (total_cents >= 0),
    created_at INTEGER NOT NULL,
    CHECK (discount_cents <= subtotal_cents),
    CHECK (total_cents = subtotal_cents - discount_cents + tax_cents)
) STRICT;

-- Order Lines
CREATE TABLE order_lines (
    order_id INTEGER NOT NULL REFERENCES orders(id),
    product_id INTEGER NOT NULL REFERENCES products(id),
    qty INTEGER NOT NULL CHECK (qty > 0),
    unit_price_cents INTEGER NOT NULL CHECK (unit_price_cents >= 0),
    PRIMARY KEY (order_id, product_id)
) STRICT;
```

## 🧪 تست‌ها

### Property-based Tests
- ✅ هزاران سناریوی رندوم برای هر تابع دامنه
- ✅ Invariant‌ها در هر سناریو چک می‌شوند
- ✅ حداقل 10000 case برای هر تابع مالی

### Integration Tests
- ✅ جریان کامل خرید
- ✅ همزمانی (چند کاربر همزمان)
- ✅ Rollback در خطا

## 📊 معیارهای پذیرش

1. ✅ همه‌ی theorem‌های Lean بدون sorry اثبات شده باشند
2. ✅ TLA+ همه‌ی invariant‌ها را پاس کند
3. ✅ کد Rust دامنه از Lean استخراج شده باشد (نه دست‌نویس)
4. ✅ SQLite با STRICT + CHECK کار کند
5. ✅ هر تراکنش invariant‌ها را runtime چک کند
6. ✅ p99 API زیر 10ms باشد
7. ✅ کل پروژه در یه binary deploy بشود
8. ✅ صفر feature اضافه داشته باشد
9. ✅ proptest با 10000+ case پاس بشود
10. ✅ litestream بکاپ بگیرد

## 📚 منابع

- [Lean 4 Manual](https://lean-lang.org)
- [Theorem Proving in Lean 4](https://leanprover.github.io/theorem_proving_in_lean4/)
- [TLA+](https://lamport.azurewebsites.net/tla/tla.html)
- [SQLite Documentation](https://www.sqlite.org/docs.html)
- [HTMX](https://htmx.org/essays)
- [Axum](https://github.com/tokio-rs/axum)
- [Pico.css](https://picocss.com)
- [Caddy](https://caddyserver.com)
- [litestream](https://litestream.io/docs)

## 🤝 مشارکت

1. مخزن را فورک کنید
2. یک شاخه جدید ایجاد کنید
3. تغییرات خود را کامیت کنید
4. به شاخه اصلی پوش کنید
5. Pull Request ایجاد کنید

## 📜 مجوز

This project is licensed under the MIT License.

---

**✨ ساخته شده با عشق برای جامعه فارسی زبان**
