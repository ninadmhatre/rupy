# Tasks

----

## CLI

1. rwjson

Desc: Read-Write JSON.

Create a basic JSON reader and writer which can write given key:val pairs to file. While writing if "key" is already present then update
the value, otherwise insert a new key=value.

Read is simple, either read full file or a single key from a file.

Usage:

```bash
rwjson --read|-r  FILE                                       # Read JSON file
rwjson --read|-r  FILE --key|-k <KEY>                        # Read top-level key value from JSON file. Handle errors!
rwjson --write|-w FILE --key|-k <KEY>:<VAL>,<KEY2>:<VAL>,... # Insert of update given keys in file. Handle errors!
```

Example:

```bash
rwjson -r ~/s.json
rwjson -r ~/s.json -k name
rwjson -w ~/s.json -k A:1,B:"S o m e t hing"
```

2. killer

Desc: Kill a process by PID or Name.

Create a small utility to kill a process by PID or Name. Keep it very simple. Its a combination of kill and pkill but not full implementation.

Usage:

```bash
killer -i PID
killer -p NAME
```

Examples:

```bash
killer -i 1234
killer -p vscode
```

3. myenv

Desc: Create, Load and View an environment file created at specific location.

This is equivalent of .dotenv, but you have 1 big JSON file with multiple envs and their environment variables.

Bonus: Make it cross system compatible. i.e. should run on Window, Linux and Mac.

Usage:

```bash
menv --create|-c NAME -v KEY=VAL,KEY2=VAL2    # creates
menv --view|-v   NAME                         # prints the values
menv --load|-l   NAME                         # loads the environment
``` 

Example:

```bash
menv -c py-a -v PYTHONPATH=$PYTHONPATH:/opt/repos/a,LOG_LEVEL=debug
menv -v py-a
menv -l py-a
```

4. fetchdb

Desc: wrapper over DB (sqlite or RDBMS) that fetches something from DB.

Prerequesite: Create a simple table with some info (student info) [name,age,class,geneder]

Usage:

```bash
fetchdb -where|-w <COND>
```

Examples:

```bash
fetchdb -w 'name = "xyz" AND age > 12'  # prints result 1 per line
```

## Library

1. Create a lib that can read/write JSON (Same as example 1 but move the read/write JSON code to another module)
2. Create a lib that can fetch data from DB (same as example 4 above, but move the code to lib) 

## Web

1. Create a MATH REST API that returns a JSON response
   - GET <url>/[add|sub|mul|div|pow]/<int1>/<int2>
   - GET <url>/[ceil|flor]/<float>

```bash
$ curl -x <url>/add/1/2
{
   "operation": "add",
   "inputs": [1, 2],
   "result": 3
}
```

2. Create a Web App that does the same as MATH REST API but using forms

3. Create a REST API that can fetch something from DB (use cli:fetchdb example and return a JSON/CSV response)
   - GET <url>/where/name.eq.xyz/age.gt.12         # eq/neq/lt/lte/gt/gte/bw/like
   - GET <url>/where/name.like.xy%/age.gte.12  

```bash
$ curl <url>/where/name.eq.XYZ?json
{"name": "XYZ", "age": 12, "class": 6, "gender": "M"}

$ curl <url>/where/name.eq.XYZ?csv
name,age,class,gender
XYZ,12,6,M
```
