"""
$ pq.py --head|-h <int> --tail|-t <int> --info|-i --query|-q <str> FILE
"""

import argparse
import pathlib as pl

import pandas as pd


def parse_args():
    parser = argparse.ArgumentParser("pq", description="Parquet CLI Tool")

    parser.add_argument("file")

    mutually_exclusive = parser.add_mutually_exclusive_group(required=True)

    mutually_exclusive.add_argument("--first", "-f", type=int)
    mutually_exclusive.add_argument("--last", "-l", type=int)
    mutually_exclusive.add_argument("--info", "-i", action="store_true")
    mutually_exclusive.add_argument("--query", "-q", type=str, default=None)

    args = parser.parse_args()

    return args


def handle(args):
    df = pd.read_parquet(pl.Path(args.file))

    if args.first:
        return df.head(args.first)

    if args.last:
        return df.tail(args.last)

    if args.info:
        return df.info()

    if args.query:
        return df.query(args.query)


def main():
    args = parse_args()
    print(handle(args))


if __name__ == "__main__":
    main()
