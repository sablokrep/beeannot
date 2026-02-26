use crate::beesearch::DNASeq;
use crate::beesearch::PathFile;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqlitePoolOptions;
use std::error::Error;

use std::path::Path;
/*
Gaurav Sablok
codeprog@icloud.com
*/

// passing a impl to the function

#[tokio::main]
pub async fn beedatabase() -> Result<String, Box<dyn Error>> {
    let pathview = Path::new("../beedata/sqldata.fasta");
    let pathopen = PathFile {
        pathfile: pathview.to_str().unwrap().to_string(),
    };
    let openpath = pathopen.prepareaxum(pathview).unwrap();
    let connection = SqliteConnectOptions::new()
        .filename("beesearch.db")
        .create_if_missing(true);
    let connect = SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(connection)
        .await?;
    sqlx::query(
        r#" CREATE TABLE IF NOT EXISTS BEEDATA(
            geneid integer primary key,
            genename text not null,
            start text not null,
            end text not null,
            idtype text not null unique,
            parentype text not null"#,
    )
    .execute(&connect)
    .await?;
    for i in openpath.iter() {
        sqlx::query(
            "INSERT INTO variants(geneid, genename, start, end, idtype, parentype)
            values ( $1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .bind(i.name.clone())
        .bind(i.geneid.clone())
        .bind(i.annotationtype.clone())
        .bind(i.start.clone())
        .bind(i.end.clone())
        .bind(i.idtype.clone())
        .bind(i.parentid.to_string().clone())
        .execute(&connect)
        .await
        .unwrap();
    }

    Ok("The database has been written".to_string())
}
