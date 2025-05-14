use crate::structures::static_vars::DB;

pub async fn get_newfeed_posts() {
    let surql = "SELECT * FROM tb_posts ORDER BY RAND() LIMIT 50;";

    let resul = DB.query(surql).await;
}
