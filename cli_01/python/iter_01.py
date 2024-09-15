"""
Function based implementation! 

It works, but i have better (some may call it over engineered) solution in iter_02!
"""
import abc
import argparse
import json
import os
from pathlib import Path
import sys
import typing as t
from functools import cache


PROFILE_FILE = "env.json"

PLATFORM_DEFAULTS = {
    "gcp": {
        "profile_dir": Path("/GCP/.rupy"),
        "root_dir": Path("/GCP/user/rupy"),
        "cache_dir": Path("/GCP/user/rupy/cache"),
        "metrics_dir": Path("/GCP/user/rupy/metrics")
    },
    "linux": {
        "profile_dir": Path("~/.rupy").expanduser(),
        "root_dir": Path("~/rupy").expanduser(),
        "cache_dir": Path("~/rupy/cache").expanduser(),
        "metrics_dir": Path("~/rupy/metrics").expanduser(),
    }
}

@cache
def profile_dir(platform) -> Path:
    return PLATFORM_DEFAULTS[platform]["profile_dir"]

@cache
def profile_file(platform) -> Path:
    return profile_dir(platform).joinpath(PROFILE_FILE)


def read_profile_file(platform) -> t.Dict:
    prof_file = profile_file(platform)
    if not prof_file.is_file():
        raise FileNotFoundError(f"{prof_file.as_posix()} does not exist!")

    return json.loads(prof_file.read_text())


def write_profile_file(platform, data: t.Dict):
    profile_file(platform).write_text(json.dumps(data, indent=4))
    print(f"Info: {profile_file(platform).as_posix()} created...")


def gcp_handler(args: t.Dict):
    cmd = args.pop('cmd')
    args['platform'] = 'gcp'

    if cmd == "init":
        return init_cmd(**args)
    
    if cmd == "load":
        return load_cmd(**args)

    if cmd == "debug":
        return debug_cmd(**args)

    raise ValueError(f"Invalid {cmd} was passed!")


def linux_handler(args: t.Dict):
    cmd = args.pop("cmd")
    args['platform'] = 'linux'

    if cmd == "init":
        return init_cmd(**args)

    if cmd == "load":
        return load_cmd(**args)

    if cmd == "debug":
        return debug_cmd(**args)

    raise ValueError(f"Invalid {cmd} was passed!")


def init_cmd(platform: str, recreate: bool, def_profile: str):
    template = {
        "profiles": {
            "common": {"A": 1, "B": True, "C": "rw"},
            "dev": {"HOST": "a", "NAME": "DB"},
            "dev-lite": {"LOCAL_DB_DIR": "/tmp/rupy", "LDB_VAR1": "r", "LDB_VAR2": "w"},
            "uat": {"LOCAL_DB_DIR": None, "HOST": None, "NAME": None},
            "default": def_profile,
        }
    }

    prof_file = profile_file(platform)

    if prof_file.is_file() and not recreate:
        print(f"Error: {prof_file.as_posix()} already exist!")
        sys.exit(0)

    profile_dir(platform).mkdir(parents=True, exist_ok=True)
    write_profile_file(platform, template)

def load_cmd(platform: str, profile: t.Optional[str]):
    all_profiles = read_profile_file(platform)["profiles"]
    profile_name = profile or all_profiles["default"]

    try:
        selected_profile_vars = all_profiles[profile_name]
    except KeyError:
        print(f"Error: Profile '{profile_name}' does not exist!")
        sys.exit(1)

    common_vars = all_profiles["common"]
    all_vars = {**common_vars, **selected_profile_vars}

    for key, val in all_vars.items():
        if val is None:
            print(f"unset {key}")
        else:
            print(f"export {key}='{val}'")

def debug_cmd(platform: str, profile: t.Optional[str]):
    all_profiles = read_profile_file(platform)["profiles"]
    profile_name = profile or all_profiles["default"]

    print(f"Available Profiles: [{all_profiles.keys()}]")
    print(f"Loading values for [{profile_name}]")
    print("-------")

    load_cmd(platform, profile_name)


def parse() -> dict:
    # Create the top-level parser
    parser = argparse.ArgumentParser(description="env setup & load tool")

    subparsers = parser.add_subparsers(dest="command", help="Sub-command help")

    _profile_choices = ["dev-lite", "dev", "uat", "prod", "test"]

    # Create the parser for the 'init' command
    parser_init = subparsers.add_parser("init", help="Initialize the environment")
    parser_init.add_argument("--recreate", "-r", action="store_true", help="Recreate the environment")
    parser_init.add_argument(
        "--default-profile", "-dp", choices=_profile_choices, default="dev", help="Set the default profile"
    )

    # Create the parser for the 'load' command
    parser_load = subparsers.add_parser("load", help="Load a profile")
    parser_load.add_argument("--profile", "-p", choices=_profile_choices, help="Specify the profile to load")

    # create the parser for 'debug' command
    parser_load = subparsers.add_parser("debug", help="Debug info about a profile")
    parser_load.add_argument("--profile", "-p", choices=_profile_choices, help="Specify the profile to load")

    # Parse the arguments
    args = parser.parse_args()

    final_args = {}
    # Execute the appropriate command
    if args.command == "init":
        final_args = {"cmd": "init", "recreate": args.recreate, "def_profile": args.default_profile}
    elif args.command in ("load", "debug"):
        final_args = {"cmd": args.command, "profile": args.profile}
    else:
        parser.print_help()
        sys.exit(1)

    return final_args


def get_platform() -> str:
    if os.environ.get("ON_GCP", "") == "1":
        return "gcp"
    return "linux"


def get_command_handler() -> t.Callable:
    platform = get_platform()
    
    return {"gcp": gcp_handler, "linux": linux_handler}[platform]


def main():
    args = parse()

    handler = get_command_handler()
    handler(args)


if __name__ == "__main__":
    main()

