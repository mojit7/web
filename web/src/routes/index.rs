//! Index Route
//!
//! Handles the main page.

use axum::response::Html;

/// Main index page
pub async fn index() -> Html<String> {
    Html(format!(
        r#"<!DOCTYPE html>
<html lang="fa" dir="rtl">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Shop-Proof - فروشگاه اثبات‌پذیر</title>
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
            <h1>به Shop-Proof خوش آمدید</h1>
            <p>یک فروشگاه آنلاین کاملا اثبات‌پذیر با معماری فرمال</p>
            <p>همه عملیات مالی با اثبات ریاضی انجام می‌شوند.</p>
            
            <article>
                <h2>ویژگی‌ها</h2>
                <ul>
                    <li>✅ پول هرگز منفی نمی‌شود</li>
                    <li>✅ تخفیف هرگز از مبلغ بیشتر نیست</li>
                    <li>✅ مالیات همیشه درست محاسبه می‌شود</li>
                    <li>✅ موجودی هرگز منفی نمی‌شود</li>
                    <li>✅ همه تراکنش‌ها اتمیک هستند</li>
                </ul>
            </article>
            
            <a href="/products" role="button">مشاهده محصولات</a>
        </section>
    </main>
</body>
</html>"#
    ))
}
