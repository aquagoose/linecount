# linecount
Count the number of lines of code in your project.

## Usage
For basic usage, its easy. Just run:

```
linecount <DIR>
```

### Excluding Directories
Excluding directories from indexing, such as a `bin` or `obj` directory, can be done as follows:

```
--exclude bin
-e obj
```

This will exclude **all** directories with that directory name.

### Only Show Certain Filetypes
By default, the counter counts **all** known filetypes. If you only want to read certain filetypes, it can be done like so:

```
--file-type cs
-f hlsl
```

### Count Comments/Whitespace
By default, the counter ignores comments and whitespace. You can count comments by appending `--count-comments`, and count whitespace by appending `--count-whitespace`.

### Counting Unknown Filetypes
If you want to count files with extensions that are unknown to the counter, append `--count-unknown`.

If there's anything that should be known by the counter, please open an issue or pull request!