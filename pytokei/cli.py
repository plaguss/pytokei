from pathlib import Path
from typing import Dict, List, Tuple

import typer
from rich.console import Console
from rich.table import Table
from rich.text import Text

import pytokei

app = typer.Typer()

ReportType = Dict[str, Dict[str, int]]

REPORT_TO_POSITION = {
    "files": 1,
    "lines": 2,
    "code": 3,
    "comments": 4,
    "blanks": 5,
}


def _report_as_list(report: ReportType) -> List[Tuple[str, int, int, int, int, int]]:
    """Transform the report to simplify sorting.

    Args:
        report (ReportType): Output from pytokei.

    Returns:
        List[List[str]]: report in a list of lists.
            The first column corresponds to the languages,
            then files, lines, code, comments and blanks.
    """
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


def to_table(
    report: ReportType,
    title: str = "pytokei report",
    colored: bool = True,
    sort: str = "lines",
) -> None:
    """Creates a rich table to print the report to the console.

    Args:
        report (Dict[str, Dict[str, int]]): pytokei's report.
        title (str, optional): Title for the table. Defaults to "pytokei report".
        colored (bool): Whether to report the table with colors or not.
        sort (str, optional): Variable to sort the table. By default is not sorted.
    """
    report_ = _report_as_list(report)

    report_ = sorted(report_, key=lambda x: x[REPORT_TO_POSITION[sort]], reverse=True)

    table = Table(title=title)
    columns = ("language", "files", "lines", "code", "comments", "blanks")

    if colored:
        colors_even = (
            "deep_sky_blue2",
            "medium_purple",
            "red",
            "gold1",
            "green3",
            "grey82",
        )
        colors_odd = (
            "deep_sky_blue3",
            "dark_violet",
            "dark_red",
            "yellow",
            "green4",
            "grey39",
        )
    else:
        colors_even = ("",) * 6
        colors_odd = colors_even

    table.add_column(
        columns[0], justify="left", style=colors_even[0], header_style=colors_even[0]
    )

    for c, color in zip(columns[1:], colors_even[1:]):
        table.add_column(
            c.capitalize(), justify="right", style=color, header_style=color
        )

    for i, data in enumerate(report_):
        colors = colors_even if (i % 2 == 0) else colors_odd
        row = [Text(text=data[0], style=colors[0])]
        for value, color in zip(data[1:], colors[1:]):
            row.append(Text(text=str(value), style=color))

        table.add_row(*row)

    console = Console()
    console.print(table)


@app.command()
def main(
    path: Path = typer.Argument(..., help="Path to the file or directory to count."),
    ignore_paths: str = typer.Option(
        "nothing",
        "--ignore-paths",
        "-i",
        help=(
            "List of paths to ignore, comma separated. For example `/docs,pyproject.toml`"
        ),
    ),
    sort: str = typer.Option(
        "lines",
        "-s",
        "--sort",
        help=f"If given, sorts the result by this value. Must be one of {set(REPORT_TO_POSITION.keys())}.",
    ),
    colored: bool = typer.Option(
        True, help="Whether to add color to the report or not."
    ),
) -> None:  # pragma: no cover
    """Pytokei Command Line Interface."""
    langs = pytokei.Languages()
    conf = pytokei.Config()  # Just use the default for now
    langs.get_statistics([str(path)], ignore_paths.split(","), conf)
    report = langs.report_compact_plain()

    to_table(report, title=str(path), colored=colored, sort=sort)
