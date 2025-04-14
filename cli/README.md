# Env Setup & Load CLI

----

### Description

A simple Profile (Env) setup & loader tool. It should handle different profiles as per the OS (Linux, Mac, Windows, etc). 

All values are stored as "key=value" as per profile in JSON file.

```python
{
  "profiles": {
    "common": {                 # available for all profiles        
      "HOST": "localhost",       
      "PORT": 3306,
      "DB_NAME": "Test"
    },
    "dev": {                    # only override "HOST" from common, define new variable
      "HOST": "dev-host"
      "ENV": "dev"
      "DEBUG": "1"
    },
    "uat": {
      "HOST": "uat-host",
      "DB_NAME": "user_acceptance"
      "ENV": "uat"
    },
    "prod": {
      "HOST": "prod-host",
      "DEBUG": None           # anything set to None/null, will be unset from environment.
    }
  }
}
```

### Usage

```python

$ cmd init [--recreate|-r] [--default-profile|-dp <name>]
$ cmd load [--profile|-p <name>]
$ cmd debug [--profile|-p <name>]
```

### Example

```shell
$ cmd init -r -d dev    # recreate a profile JSON file with default profile "dev"
$ cmd load              # prints export <VAR>=<VAL> for default profile per line which needs to be fed into source for setting up.
$ cmd load -p dev_lite  # same as above but for "dev_lite" profile
# cmd debug -p dev_lite # prints all <KEY>=<VAL> on screen 
```
