//! Orders Routes
//!
//! Handles order-related routes.

use axum::{
    extract::{Form, Path},
    response::Html,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateOrderForm {
    // In a real implementation, this would contain cart data
}

/// Create a new order
pub async fn create_order(Form(_form): Form<CreateOrderForm>) -> Html<String> {
    // In a real implementation, this would create an order from the cart
    // For now, just show a confirmation page
    Html(format!(
        r#"<!DOCTYPE html>
<html lang="fa" dir="rtl">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>تایید سفارش - Shop-Proof</title>
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
            <h1>تایید سفارش</h1>
            
            <article>
                <h2>سفارش شما ثبت شد!</h2>
                <p>شماره سفارش: #12345</p>
                <p>مبلغ کل: 100,000 تومان</p>
                <p>از خرید شما سپاسگزاریم.</p>
            </article>
            
            <a href="/" role="button">بازگشت به صفحه اصلی</a>
        </section>
    </main>
</body>
</html>"#
    ))
}

/// Get order details
pub async fn get_order(Path(id): Path<u64>) -> Html<String> {
    Html(format!(
        r#"<!DOCTYPE html>
<html lang="fa" dir="rtl">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>جزئیات سفارش - Shop-Proof</title>
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
            <h1>جزئیات سفارش #{}</h1>
            
            <article>
                <h2>سفارش #12345</h2>
                <p>وضعیت: تایید شده</p>
                <p>تاریخ: 1403/07/20</p>
                <p>مبلغ کل: 100,000 تومان</p>
                
                <h3> آیتم‌ها </h3>
                <table>
                    <thead>
                        <tr>
                            <th>محصول</th>
                            <th>تعداد</th>
                            <th>قیمت</th>
                            <th>جمع</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>محصول نمونه</td>
                            <td>2</td>
                            <td>50,000 تومان</td>
                            <td>100,000 تومان</td>
                        </tr>
                    </tbody>
                </table>
            </article>
            
            <a href="/" role="button">بازگشت به صفحه اصلی</a>
        </section>
    </main>
</body>
</html>"#,
        id
    ))
}
