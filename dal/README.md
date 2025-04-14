
# dal

----

>> !! In Progress !!

### Description

Build a Data Access Layer around DuckDB to perform select, insert, update & delete (CRUD) operations. You can add assume
insert could be a bulk insert into a table.

This is a library.

I plan to use SQLite as I never used it, so good chance to learn something new.

There will be only 1 table (Trading Prices, 1 entry per day for every symbol), schema of that table.

```python
class Prices(Table):
    id: int
    symbol: str
    date: date
    close: float
```

Tasks:
1. Setup DB if not already setup.
2. Insert Mock data
3. Query data 
   4. valid - query for a AAPL for a date, range of dates 
   5. invalid - query for non-existing symbol
4. Insert data
   5. single - insert new entry for an existing symbol for a day 
   6. bulk - insert new symbol with range
7. Update 
   8. update price for a given symbol at given date
9. Delete 
   10. Delete one entry by symbol and date
   11. Delete multiple entries for symbol and date range
12. Use Dataframes (use Polars in both python and rust)

### Examples

----

```python
from dal import setup, select, insert, update, delete

def setup_db(file_path: str):
    setup(file_path)
    

def get_px_for_date(symbol: str, day: date) -> DF:...
def get_px_for_dates(symbol: str, start: date, end: date) -> DF: ...
def add_px_for_date(symbol: str, px: float, day: date) -> int: ... 
def add_px_for_dates(symbol: str, px: list(float), start: date, end: date) -> int: ...
def set_px_for_date(symbol: str, new_px: float, day: date) -> bool: ...
def del_px_for_date(symbol: str, day: date) -> bool: ...
def del_px_for_dates(symbol: str, start: date, end: date) -> bool: ...
```
