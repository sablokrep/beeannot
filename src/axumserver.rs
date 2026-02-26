use axum::extract::Path;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqlitePoolOptions;
use std::fs::File;
use std::io::Write;

use crate::beesearch::DNASeqUnfold;

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub async fn variant_id(Path(beesearchid): Path<String>) -> String {
    let connection = SqliteConnectOptions::new()
        .filename("variantsearch.db")
        .create_if_missing(true);
    let connect = SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(connection)
        .await
        .unwrap();
    let rows: Vec<DNASeqUnfold> = sqlx::query_as("SELECT name,geneid, annotationtype, start, end, idtype, parentid FROM BEESTORAGE WHERE id = $1").bind(variantsearch)
        .fetch_all(&connect)
        .await.unwrap();
    let mut filewrite = File::create("fileidvariant.txt").expect("file not present");
    for i in rows.iter() {
        writeln!(
            filewrite,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            i.filename, i.pos, i.chrom, i.id, i.refallele, i.altallele, i.quality, i.filter
        )
        .expect("file not present");
    }

    "The results of the fetched id are given above".to_string()
}
