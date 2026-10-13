//! Admin Routes
//!
//! Handles admin-related routes.

use axum::{
    extract::Form,
    response::Html,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AddProductForm {
    pub name: String,
    pub price_cents: i64,
}

#[derive(Debug, Deserialize)]
pub struct SetStockForm {
    pub product_id: u64,
    pub qty: u64,
}

/// Admin login page
pub async fn login() -> Html<String> {
    Html(format!(
        r#"<!DOCTYPE html>
<html lang="fa" dir="rtl">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>ورود ادمین - Shop-Proof</title>
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
            </ul>
        </nav>
        
        <section>
            <h1>ورود به پنل مدیریت</h1>
            
            <form>
                <label for="username">نام کاربری</label>
                <input type="text" id="username" name="username" required>
                
                <label for="password">رمز عبور</label>
                <input type="password" id="password" name="password" required>
                
                <button type="submit">ورود</button>
            </form>
        </section>
    </main>
</body>
</html>"#
    ))
}

/// List products (admin)
pub async fn list_products() -> Html<String> {
    Html(format!(
        r#"<!DOCTYPE html>
<html lang="fa" dir="rtl">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>مدیریت محصولات - Shop-Proof</title>
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
                <li><a href="/admin/products">مدیریت محصولات</a></li>
                <li><a href="/admin/stock">مدیریت موجودی</a></li>
            </ul>
        </nav>
        
        <section>
            <h1>مدیریت محصولات</h1>
            
            <form hx-post="/admin/products" hx-swap="outerHTML">
                <label for="name">نام محصول</label>
                <input type="text" id="name" name="name" required>
                
                <label for="price_cents">قیمت (تومان)</label>
                <input type="number" id="price_cents" name="price_cents" required min="0">
                
                <button type="submit">افزودن محصول</button>
            </form>
            
            <table>
                <thead>
                    <tr>
                        <th>ID</th>
                        <th>نام</th>
                        <th>قیمت</th>
                        <th>عملیات</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>1</td>
                        <td>محصول نمونه</td>
                        <td>100,000 تومان</td>
                        <td><a href="#">ویرایش</a> | <a href="#">حذف</a></td>
                    </tr>
                </tbody>
            </table>
        </section>
    </main>
</body>
</html>"#
    ))
}

/// Add product (admin)
pub async fn add_product(Form(form): Form<AddProductForm>) -> Html<String> {
    // In a real implementation, this would add a product to the database
    // For now, just redirect back
    Html(format!(
        r#"<div 
            hx-get="/admin/products" 
            hx-trigger="load" 
            hx-swap="outerHTML" 
            hx-push-url="/admin/products"
        ></div>"#
    ))
}

/// Set stock (admin)
pub async fn set_stock(Form(form): Form<SetStockForm>) -> Html<String> {
    // In a real implementation, this would update stock in the database
    // For now, just redirect back
    Html(format!(
        r#"<div 
            hx-get="/admin/products" 
            hx-trigger="load" 
            hx-swap="outerHTML" 
            hx-push-url="/admin/products"
        ></div>"#
    ))
}
