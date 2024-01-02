/// Get all todos
pub async fn all(page: u32) -> Result<ArticleListInfo, Error> {
    Request::get("http://localhost:3000/api/todos")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
}
