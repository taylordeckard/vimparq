" Autocmd to detect .parquet files
augroup ParquetEdit
  autocmd!
  autocmd BufReadPost *.parquet call ParquetView(expand("%:p"))
  autocmd BufNewFile *.parquet call ParquetCreate(expand("%:p"))
  autocmd BufWriteCmd *.vimparq.jsonl call ParquetSave(expand("%:p"))
augroup END

function! ParquetView(filepath)
  let tmpfile = tempname() . ".vimparq.jsonl"
  let cmd = "vimparq view " . shellescape(a:filepath) . " > " . shellescape(tmpfile)
  let output = system(cmd)
  let exit_code = v:shell_error
  
  if exit_code != 0
    echoerr "Failed to view Parquet file: " . output
    return
  endif
  
  if !filereadable(tmpfile)
    echoerr "Failed to create temporary file for editing"
    return
  endif
  
  silent! execute 'edit ' . tmpfile
  setlocal filetype=json
  setlocal nowrap
  setlocal syntax=off
  let b:parquet_original = a:filepath
  let b:parquet_tmpfile = tmpfile
  echom "Parquet file loaded successfully"
endfunction

function! ParquetCreate(filepath)
  " Called when a new *.parquet file is opened
  let tmpfile = tempname() . ".vimparq.jsonl"
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
    let exit_code = v:shell_error
    
    if exit_code != 0
      echoerr "Failed to update Parquet file: " . output
      return
    endif
    
    echom "✅ Updated Parquet: " . b:parquet_original
    " Optionally clean up
    call delete(b:parquet_tmpfile)
    setlocal nomodified
  else
    echoerr "No edit buffer associated with a Parquet file!"
  endif
endfunction