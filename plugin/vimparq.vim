" Autocmd to detect .parquet files
augroup ParquetEdit
  autocmd!
  autocmd BufReadPost *.parquet call ParquetView(expand("%:p"))
  autocmd BufWriteCmd *.vimparq.jsonl call ParquetSave(expand("%:p"))
augroup END

function! ParquetView(filepath)
  let tmpfile = tempname() . ".vimparq.jsonl"
  let cmd = "vimparq view " . shellescape(a:filepath) . " > " . shellescape(tmpfile)
  call system(cmd)
  silent! execute 'edit ' . tmpfile
  setlocal filetype=json
  setlocal nowrap
  setlocal syntax=off
  let b:parquet_original = a:filepath
  let b:parquet_tmpfile = tmpfile
endfunction

function! ParquetSave(filepath)
  if exists("b:parquet_tmpfile") && exists("b:parquet_original")
    echom "🛠 ParquetSave triggered"
    " Save buffer contents to the temp file
    write!
    let cmd = "vimparq edit " . shellescape(b:parquet_original) . " " . shellescape(b:parquet_tmpfile)
    let output = system(cmd)
    echom "Updated Parquet: " . b:parquet_original
    " Optionally clean up
    call delete(b:parquet_tmpfile)
    " Reload original .parquet if needed
  else
    echom "No edit buffer associated!"
  endif
endfunction
