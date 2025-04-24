use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};
use std::path::PathBuf;
use std::error::Error;
use std::sync::Arc;

use arrow::json::reader::{ReaderBuilder, infer_json_schema_from_seekable};
use parquet::arrow::ArrowWriter;

pub fn edit_parquet(path: &PathBuf, json_path: &PathBuf) -> Result<(), Box<dyn Error>> {

    // Open JSONL file
    let json_file = File::open(json_path)?;
    let mut buf_reader = BufReader::new(json_file);
    buf_reader.seek(SeekFrom::Start(0))?;

    // Infer schema
    let (schema, _) = infer_json_schema_from_seekable(&mut buf_reader, Some(1))?;
    let schema = Arc::new(schema);

    // Read JSON using the schema
    let builder = ReaderBuilder::new(schema.clone());
    let mut json_reader = builder.build(buf_reader)?;

    // Overwrite Parquet file
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
