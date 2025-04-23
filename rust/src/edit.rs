use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};
use std::path::PathBuf;
use std::error::Error;

use arrow::datatypes::SchemaRef;
use arrow_json::reader::ReaderBuilder;
use parquet::arrow::ArrowWriter;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

pub fn edit_parquet(path: &PathBuf, json_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    // Step 1: Open Parquet file and get schema
    let parquet_file = File::open(path)?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(parquet_file)?;
    let schema: SchemaRef = builder.schema().clone();  // get_schema() is gone in v55

    // Step 2: Open JSONL file
    let json_file = File::open(json_path)?;
    let mut buf_reader = BufReader::new(json_file);
    buf_reader.seek(SeekFrom::Start(0))?;

    // Step 3: Read JSON using the schema
    let builder = ReaderBuilder::new(schema.clone());
    let mut json_reader = builder.build(buf_reader)?;

    // Step 4: Overwrite Parquet file
    let out_file = File::create(path)?;
    println!("Writing to: {:?}", out_file);
    let mut writer = ArrowWriter::try_new(out_file, schema, None)?;

    while let Some(batch_result) = json_reader.next() {
        let batch = batch_result?;
        writer.write(&batch)?;
    }

    writer.close()?;
    Ok(())
}
