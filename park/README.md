
# park

----

### Description

Parque file utility.

Create a simple Parquet file reader. It should be able to do head, tail, schema and single col query without needing to open the file.


### Usage

----

```bash
$ park FILE               # show first 10 rows
$ park -t 10 FILE         # show last 10 rows
$ park -i FILE            # show schema, row count of the file
$ park -q "<query>" FILE  # single col query on FILE
```

### Example

----

```bash
$ park a.parquet                 # park --head|-h 10 a.parquet
$ park --tail|-t 10 a.parquet  
$ park --info|-i a.parquet       # show schema, row count etc...
$ park -q "age > 10" a.parquet   # quick query parqut file on single column.
```
