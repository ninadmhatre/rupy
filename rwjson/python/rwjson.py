import json
import typing as t
import argparse
import pathlib as pl


def parse_args() -> argparse.Namespace:
    def _is_file(val):
        val = pl.Path(val)
        if not val.is_file():
            raise argparse.ArgumentTypeError(f"{val} must exist!")
        return val

    def _as_file(val):
        return pl.Path(val)

    parser = argparse.ArgumentParser("rwjson")

    parser.add_argument("--read", "-r", type=_is_file, help="read a given json file")
    parser.add_argument("--write", "-w", type=_as_file, help="write / update json file")
    parser.add_argument("--key", "-k", help="optional for reading, mandatory while writing")

    args = parser.parse_args()

    if not (args.read or args.write):
        raise Exception("Either --read/-r or --write/-w must be provided!")

    if args.write and not args.key:
        raise Exception("For --write/-w, --keys must be specified!")

    return args

def _read(filepath: pl.Path, key: t.Optional[str]):
    data = json.loads(filepath.read_text())

    if key:
        data = data[key]

    print(json.dumps(data, indent=4))


def _write(filepath: pl.Path, keys: t.Dict[str, str]):
    if filepath.exists():
        data = json.loads(filepath.read_text())
    else:
        data = {}

    for k, v in keys.items():
        data[k] = v

    filepath.write_text(json.dumps(data, indent=4))


def run(args: argparse.Namespace):
    if args.read:
        _read(args.read, args.key)

    if args.write:
        pairs = args.key.split(",")

        keys = {}
        for i in pairs:
            k, v = i.split(":", maxsplit=1)
            keys[k] = v

        _write(args.write, keys)


if __name__ == '__main__':
    run(parse_args())