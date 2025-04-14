"""
Not Working!
"""

import pathlib
import typing as t
import pathlib as path
from dataclasses import dataclass
import datetime as dt
import random


import polars as pl
import sqlalchemy as sa

DF = pl.DataFrame

class DAL:
    def __init__(self, db_dir: path.Path, file_name: str = "dal.db"):
        self.db_dir = db_dir
        self.file_name = file_name
        self.db_file = self.db_dir.joinpath(self.file_name)
        self.create()

    def get_connection_uri(self) -> str:
        return f"sqlite://{self.db_file.as_posix()}"

    def get_db_engine(self) -> sa.Engine:
        return sa.create_engine(self.get_connection_uri())

    def run_read_query(self, query, params: t.Optional[t.Tuple[t.Any, ...]] = None) -> DF:
        with self.get_db_engine().connect() as conn:
            result = pl.read_database(query, connection=conn, execute_options={"parameters": params})

        return result

    def run_write_query(self, query: str, params: t.Optional[t.Tuple[t.Any, ...]] = None) -> t.Any:
        with self.get_db_engine().connect() as conn:
            result = conn.execute(sa.text(query), parameters=params).fetchall()

        return result

    # def write_tbl(self, df: DF):
    #     with self.get_db_engine(read_only=False) as conn:
    #         result = df.write_database(table_name="prices", connection=conn, if_table_exists="append")
    #
    #     return result

    def create(self):
        """ create table """
        if not self.db_file.exists():
            self.db_file.parent.mkdir(parents=True, exist_ok=True)

        qry = """CREATE TABLE IF NOT EXISTS prices(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            symbol VARCHAR PRIMARY KEY NOT NULL,
            date DATE NOT NULL,
            close FLOAT NOT NULL) 
        """

        self.run_write_query(qry)

    def get_px_for_date(self, symbol: str, day: dt.date) -> DF:
        qry = """SELECT symbol, date, close FROM prices WHERE symbol=? AND date=?"""
        params = (symbol, day)

        return self.run_read_query(qry, params)

    def get_px_for_dates(self, symbol: str, start: dt.date, end: dt.date) -> DF:
        qry = "SELECT symbol, date, close FROM prices WHERE symbol=? AND date>=? AND date<=?"
        params = (symbol, start, end)

        return self.run_read_query(qry, params)

    def add_px_for_date(self, symbol: str, px: float, day: dt.date) -> int:
        qry = "INSERT INTO prices(symbol, date, close) VALUES (?, ?, ?)"
        params = (symbol, day, px)
        return self.run_write_query(qry, params)

    def add_px_for_dates(self, symbol: str, px: t.List[float], start: dt.date, end: dt.date) -> int:
        dates = pl.date_range(start, end, closed="both", eager=True)
        assert len(dates) == len(px), f"prices for some dates are missing! {len(dates)=} vs. {len(px)=}"

        _symbol = [symbol] * len(px)
        data = zip(_symbol, dates, px)
        df = pl.DataFrame(data=data, schema=["symbol", "date", "close"])

        self.write_tbl(df)

    def set_px_for_date(self, symbol: str, new_px: float, day: dt.date) -> bool: ...
    def del_px_for_date(self, symbol: str, day: dt.date) -> bool: ...
    def del_px_for_dates(self, symbol: str, start: dt.date, end: dt.date) -> bool: ...

def populate_db(d: DAL):
    symbols = {"AAPL": 100, "MSFT": 80, "GOOG": 60}
    start_date = dt.date(2020, 1, 1)
    end_date = dt.date(2025, 1, 1)

    data = []

    for day in pl.date_range(start_date, end_date, closed="both", eager=True):
        direction = "down" if day.weekday() in (0, 2, 4) else "up"

        for symbol in symbols:
            move = random.uniform(0, .7)
            if direction == "up":
                symbols[symbol] += move
            else:
                symbols[symbol] -= move

            data.append((symbol, day, symbols[symbol]))

    df = pl.DataFrame(data=data, schema=["symbol", "date", "close"])
    print(df)

    df.write_database(table_name="prices", if_table_exists="append", connection=d.get_db_engine().connect())
    # curr_time = int(dt.datetime.now().timestamp())
    # parquet_src = f"/tmp/{curr_time}.parquet"
    # df.write_parquet(parquet_src)
    #
    # conn = d.get_db_engine()
    #
    # conn.executemany(query="INSERT INTO prices(symbol, date, close) VALUES (?, ?, ?)", parameters=data)

def trial():
    d = DAL(db_dir=path.Path("/tmp"))

    populate_db(d)

    # print(d.get_px_for_date(symbol="AAPL", day=dt.date(2020, 1, 1)))
    # print(d.get_px_for_dates(symbol="AAPL", start=dt.date(2024, 1, 1), end=dt.date(2025, 1, 1)))

if __name__ == "__main__":
    trial()