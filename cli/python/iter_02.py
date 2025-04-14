"""
This is OOPs based implementation for the same problem.
"""

import abc
import argparse
import json
import os
from pathlib import Path
import sys
import typing as t


PROFILE_FILE = "env.json"


class Command(abc.ABC):
    def __init__(self, args: t.Dict, platform_defaults: t.Dict):
        self.args = args
        self.platform_defaults = platform_defaults

    @abc.abstractmethod
    def run(self):
        pass

    @property
    def profile_dir(self) -> Path:
        return self.platform_defaults["profile_dir"]

    @property
    def profile_file(self) -> Path:
        return self.profile_dir.joinpath(PROFILE_FILE)

    def read_profile_file(self) -> t.Dict:
        if not self.profile_file.is_file():
            raise FileNotFoundError(f"{self.profile_file.as_posix()} does not exist!")

        return json.loads(self.profile_file.read_text())

    def write_profile_file(self, data: t.Dict):
        self.profile_file.write_text(json.dumps(data, indent=4))
        print(f"Info: {self.profile_file.as_posix()} created...")


class InitCommand(Command):
    def __init__(self, args: t.Dict, platform_defaults: t.Dict):
        super().__init__(args, platform_defaults)

        self.recreate = args["recreate"]
        self.default_profile = args["def_profile"]

        self.profiles = ["dev-lite", "dev", "uat", "prod", "test"]

    def run(self):
        template = {
            "profiles": {
                "common": {"A": 1, "B": True, "C": "rw"},
                "dev": {"HOST": "a", "NAME": "DB"},
                "dev-lite": {"LOCAL_DB_DIR": "/tmp/rupy", "LDB_VAR1": "r", "LDB_VAR2": "w"},
                "uat": {"LOCAL_DB_DIR": None, "HOST": None, "NAME": None},
                "default": self.default_profile,
            }
        }

        if self.profile_file.is_file() and not self.recreate:
            print(f"Error: {self.profile_file.as_posix()} already exist!")
            sys.exit(0)

        self.profile_dir.mkdir(parents=True, exist_ok=True)
        self.write_profile_file(template)


class LoadCommand(Command):
    def __init__(self, args: t.Dict, platform_defaults: t.Dict):
        super().__init__(args, platform_defaults)

        self.all_profiles = self.read_profile_file()["profiles"]
        self._profile_name = args["profile"]

    @property
    def profile_name(self) -> str:
        return self._profile_name or self.all_profiles["default"]

    def get_profile_vars(self) -> dict:
        common_vars = self.all_profiles["common"]
        try:
            selected_profile_vars = self.all_profiles[self.profile_name]
        except KeyError:
            print(f"Error: Profile '{self.profile_name}' does not exist!")
            sys.exit(1)

        return {**common_vars, **selected_profile_vars}

    def run(self):
        for key, val in self.get_profile_vars().items():
            if val is None:
                print(f"unset {key}")
            else:
                print(f"export {key}='{val}'")


class DebugCommand(LoadCommand):
    def run(self):
        print(f"Available Profiles: [{self.all_profiles.keys()}]")
        print(f"Loading values for [{self.profile_name}]")
        print("-------")
        super().run()


class Platform(t.Protocol):
    profile_dir: Path
    root_dir: Path
    cache_dir: Path
    metrics_dir: Path

    commands: t.Dict[str, t.Type[Command]] = {"init": InitCommand, "load": LoadCommand, "debug": DebugCommand}

    def defaults(self) -> t.Dict:
        return {
            "profile_dir": self.profile_dir,
            "root_dir": self.root_dir,
            "cache_dir": self.cache_dir,
            "metrics_dir": self.metrics_dir,
        }

    def get_handler(self, cmd: str, args: t.Dict) -> Command:
        cls = self.commands[cmd]
        obj = cls(args, self.defaults())

        return obj


class GCP(Platform):
    profile_dir = Path("/GCP/.rupy")
    root_dir = Path("/GCP/user/rupy")
    cache_dir = root_dir.joinpath("cache")
    metrics_dir = root_dir.joinpath("metrics")


class LinuxMachine(Platform):
    profile_dir = Path("~/.rupy").expanduser()
    root_dir = Path("/opt/rupy")
    cache_dir = root_dir.joinpath("cache")
    metrics_dir = root_dir.joinpath("metrics")


class Debian(LinuxMachine):
    pass


class Windows(Platform):
    profile_dir = Path(r'c:\\\\users\\.rupy')
    root_dir = Path(r"c:\\\\users\\rupy")
    cache_dir = root_dir.joinpath("cache")
    metrics_dir = root_dir.joinpath("metrics")

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


def get_platform() -> Platform:
    if os.environ.get("ON_GCP", "") == "1":
        return GCP()
    return LinuxMachine()


def get_command_handler(cmd: str, args: t.Dict) -> Command:
    platform = get_platform()
    return platform.get_handler(cmd, args)


def main():
    args = parse()

    handler = get_command_handler(args.pop("cmd"), args)
    handler.run()


if __name__ == "__main__":
    main()

