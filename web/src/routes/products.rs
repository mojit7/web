//! Products Routes
//!
//! Handles product-related routes.

use axum::{
    extract::Path,
    response::Html,
    Json,
};
use shop_proof_core::domain::money::Money;
use shop_proof_core::domain::product::{Product, ProductId};
use std::sync::Arc;

/// List all products
pub async fn list_products() -> Html<String> {
    // In a real implementation, this would query the database
    // For now, return a simple HTML page
    Html(format!(
        r#"<!DOCTYPE html>
<html lang="fa" dir="rtl">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>محصولات - Shop-Proof</title>
    <script src="https://unpkg.com/htmx.org@1.9.10"></script>
    <link rel="stylesheet" href="https://unpkg.com/@picocss/pico@1.5.10/css/pico.min.css">
</head>
<body>
    <main class="container">
        <nav class="navbar">
            <ul class="navbar-list">
                <li><strong>Shop-Proof</strong></li>
            </ul>
            <ul class="navbar-list">
                <li><a href="/">خانه</a></li>
                <li><a href="/products">محصولات</a></li>
                <li><a href="/cart">سبد خرید</a></li>
                <li><a href="/admin/login">مدیریت</a></li>
            </ul>
        </nav>
        
        <section>
            <h1>محصولات</h1>
            
            <div 
                hx-get="/api/products" 
                hx-trigger="load" 
                hx-swap="innerHTML"
            >
                <div class="loading">در حال بارگذاری...</div>
            </div>
            
            <a href="/cart" role="button">مشاهده سبد خرید</a>
        </section>
    </main>
</body>
</html>"#
    ))
}

/// Get a specific product
pub async fn get_product(Path(id): Path<u64>) -> Html<String> {
    // In a real implementation, this would query the database
    // For now, return a simple HTML page
    Html(format!(
        r#"<!DOCTYPE html>
<html lang="fa" dir="rtl">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>محصول - Shop-Proof</title>
    <script src="https://unpkg.com/htmx.org@1.9.10"></script>
    <link rel="stylesheet" href="https://unpkg.com/@picocss/pico@1.5.10/css/pico.min.css">
</head>
<body>
    <main class="container">
        <nav class="navbar">
            <ul class="navbar-list">
                <li><strong>Shop-Proof</strong></li>
            </ul>
            <ul class="navbar-list">
                <li><a href="/">خانه</a></li>
                <li><a href="/products">محصولات</a></li>
                <li><a href="/cart">سبد خرید</a></li>
                <li><a href="/admin/login">مدیریت</a></li>
            </ul>
        </nav>
        
        <section>
            <h1>جزئیات محصول #{}</h1>
            
            <article>
                <h2>محصول نمونه</h2>
                <p>قیمت: 10,000 تومان</p>
                <p>موجودی: 100 عدد</p>
                
                <form hx-post="/cart/add" hx-swap="outerHTML">
                    <input type="hidden" name="product_id" value="{}">
                    <input type="number" name="qty" value="1" min="1">
                    <button type="submit">افزودن به سبد خرید</button>
                </form>
            </article>
            
            <a href="/products" role="button">بازگشت به لیست محصولات</a>
        </section>
    </main>
</body>
</html>"#,
        id, id
    ))
}
