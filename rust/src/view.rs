use std::fs::File;
use std::path::PathBuf;
use arrow::record_batch::RecordBatch;
use arrow::json::LineDelimitedWriter;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

pub fn view_parquet(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let reader = builder.build()?;
    let batches: Vec<RecordBatch> = reader.collect::<std::result::Result<_, _>>()?;

    write_json_lines(&batches)
}

fn write_json_lines(batches: &[RecordBatch]) -> Result<(), Box<dyn std::error::Error>> {
    // Lock stdout (or use any Write impl—File, Vec<u8>, etc.)
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    // Create a JSON-Lines writer over that handle
    let mut writer = LineDelimitedWriter::new(&mut handle);

    // Write each RecordBatch
    // The write_batches method takes &[&RecordBatch]
    let batch_refs: Vec<&RecordBatch> = batches.iter().collect();
    writer.write_batches(&batch_refs)?;
    
    // Finish to flush any buffered output
    writer.finish()?;

    Ok(())
}
