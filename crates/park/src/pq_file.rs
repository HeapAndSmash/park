use polars::prelude::*;

pub fn get_parquet_schema(pq_input_file: String) {
    let mut file = std::fs::File::open(pq_input_file).unwrap();

    let df = ParquetReader::new(&mut file).finish().unwrap();

    let df_schema = df.schema();
    dbg!(df_schema);
}
