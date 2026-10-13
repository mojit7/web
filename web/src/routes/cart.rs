//! Cart Routes
//!
//! Handles cart-related routes.

use axum::{
    extract::Form,
    response::Html,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AddToCartForm {
    pub product_id: u64,
    pub qty: u64,
}

#[derive(Debug, Deserialize)]
pub struct RemoveFromCartForm {
    pub product_id: u64,
}

/// View cart
pub async fn view_cart() -> Html<String> {
    Html(format!(
        r#"<!DOCTYPE html>
<html lang="fa" dir="rtl">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>سبد خرید - Shop-Proof</title>
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
            <h1>سبد خرید</h1>
            
            <div 
                hx-get="/api/cart" 
                hx-trigger="load" 
                hx-swap="innerHTML"
            >
                <div class="loading">در حال بارگذاری...</div>
            </div>
            
            <div>
                <a href="/products" role="button">ادامه خرید</a>
                <a href="/order" role="button">تسویه حساب</a>
            </div>
        </section>
    </main>
</body>
</html>"#
    ))
}

/// Add to cart
pub async fn add_to_cart(Form(form): Form<AddToCartForm>) -> Html<String> {
    // In a real implementation, this would add to the session cart
    // For now, just redirect back to products
    Html(format!(
        r#"<div 
            hx-get="/cart" 
            hx-trigger="load" 
            hx-swap="outerHTML" 
            hx-push-url="/cart"
        ></div>"#
    ))
}

/// Remove from cart
pub async fn remove_from_cart(Form(form): Form<RemoveFromCartForm>) -> Html<String> {
    // In a real implementation, this would remove from the session cart
    // For now, just return empty
    Html(String::new())
}
