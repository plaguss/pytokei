import argparse
from typing import Dict, List, Tuple

import pytokei

ReportType = Dict[str, Dict[str, int]]

REPORT_TO_POSITION = {
    "files": 1,
    "lines": 2,
    "code": 3,
    "comments": 4,
    "blanks": 5,
}

COLUMNS = ("language", "files", "lines", "code", "comments", "blanks")


def _report_as_list(report: ReportType) -> List[Tuple[str, int, int, int, int, int]]:
    return [
        (
            lang,
            result["files"],
            result["lines"],
            result["code"],
            result["comments"],
            result["blanks"],
        )
        for lang, result in report.items()
    ]


def to_table(report: ReportType, title: str = "pytokei report", sort: str = "lines") -> None:
    rows = _report_as_list(report)
    rows = sorted(rows, key=lambda x: x[REPORT_TO_POSITION[sort]], reverse=True)

    col_widths = [len(c) for c in COLUMNS]
    for row in rows:
        col_widths[0] = max(col_widths[0], len(row[0]))
        for i in range(1, 6):
            col_widths[i] = max(col_widths[i], len(str(row[i])))

    def fmt_row(cells: tuple) -> str:
        parts = [str(cells[0]).ljust(col_widths[0])]
        parts += [str(cells[i]).rjust(col_widths[i]) for i in range(1, 6)]
        return "  ".join(parts)

    header = fmt_row(COLUMNS)
    separator = "-" * len(header)

    print(f"\n{title}\n")
    print(header)
    print(separator)
    for row in rows:
        print(fmt_row(row))
    print()


def main() -> None:  # pragma: no cover
    """Pytokei Command Line Interface."""
    parser = argparse.ArgumentParser(
        prog="pytokei",
        description="Count lines of code in a file or directory.",
    )
    parser.add_argument("path", help="Path to the file or directory to count.")
    parser.add_argument(
        "--ignore-paths",
        "-i",
        default="nothing",
        metavar="PATHS",
        help="Comma-separated list of paths to ignore (e.g. /docs,pyproject.toml).",
    )
    parser.add_argument(
        "--sort",
        "-s",
        default="lines",
        choices=list(REPORT_TO_POSITION.keys()),
        help="Sort the result by this column (default: lines).",
    )

    args = parser.parse_args()

    langs = pytokei.Languages()
    conf = pytokei.Config()
    langs.get_statistics([args.path], args.ignore_paths.split(","), conf)
    report = langs.report_compact_plain()

    to_table(report, title=args.path, sort=args.sort)
