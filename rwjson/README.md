
# rwjson

----

### Description

Read-Write JSON.

Create a basic JSON reader and writer which can write given key:val pairs to file. While writing if "key" is already present then update
the value, otherwise insert a new key=value.

Read is simple, either read full file or a single key from a file.

### Usage

----

```bash
rwjson --read|-r  FILE                                       # Read JSON file
rwjson --read|-r  FILE --key|-k <KEY>                        # Read top-level key value from JSON file. Handle errors!
rwjson --write|-w FILE --key|-k <KEY>:<VAL>,<KEY2>:<VAL>,... # Insert of update given keys in file. Handle errors!
```

### Example

----

```bash
rwjson -r ~/s.json
rwjson -r ~/s.json -k name
rwjson -w ~/s.json -k A:1,B:"S o m e t hing"
```
