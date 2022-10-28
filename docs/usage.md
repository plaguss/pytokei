# Overview

This section guides you through the functionality available by inspecting some examples.

Lets start by importing the library, and grab the statistics on this repo:

```python
>>> import pytokei
>>> langs = pytokei.Languages()
>>> conf = pytokei.Config()
>>> langs.get_statistics(["."], ["ignored"], conf)
```

At this point, the data is processed and stored internally, so lets explore it. What languages were detected?

*If you have [rich](https://github.com/Textualize/rich) installed, the console outputs will be much nicer!*

```python
>>> from rich import print
>>> langs.language_names()
['Autoconf', 'Dockerfile', 'Makefile', 'Markdown', 'Python', 'Rust', 'Plain Text', 'TOML', 'YAML']
```

Lets get a small subset of the project, ignoring some of the folders:

```python
>>> from pytokei import Languages()
>>> langs_short = Languages()
>>> langs_short.get_statistics(["."], ["docs", "tests", "requirements"], conf)
>>> langs_short.language_names()
['Makefile', 'Markdown', 'Python', 'Rust', 'TOML', 'YAML']
```

Now we can see what the default output of tokei would contain, directly as a
python `dict`.

The following function shows the report by language compacted, using python builtin objects:

```python
>>> print(langs_short.report_compact_plain())
{
    'Markdown': {'lines': 46, 'files': 1, 'blanks': 21, 'comments': 25, 'code': 0},
    'Rust': {'code': 317, 'lines': 363, 'files': 7, 'blanks': 23, 'comments': 23},
    'Makefile': {'lines': 26, 'comments': 0, 'files': 1, 'blanks': 8, 'code': 18},
    'TOML': {'code': 14, 'lines': 20, 'comments': 2, 'blanks': 4, 'files': 2},
    'YAML': {'code': 63, 'lines': 69, 'files': 1, 'comments': 0, 'blanks': 6},
    'Python': {'comments': 0, 'files': 1, 'lines': 21, 'blanks': 1, 'code': 20}
}
```

## Accessing the internal info

After calling `get_statistics` the content is stored internally.
We can see what languages are stored internally by calling `get_languages`:

```python
>>> print(langs_short.get_languages())
{
    LanguageType(Markdown): Language(empty: false),
    LanguageType(Rust): Language(empty: false),
    LanguageType(Toml): Language(empty: false),
    LanguageType(Makefile): Language(empty: false),
    LanguageType(Yaml): Language(empty: false),
    LanguageType(Python): Language(empty: false)
}
```

Some methods have an equivalent method wich ends in `_plain`, which will return the content in builtin python objects instead of the wrapped classes. Lets see the case of `get_languages_plain`:

```python
>>> print(langs_short.get_languages_plain())
{
    'TOML': [{'./pyproject.toml': {'blanks': 4, 'comments': 0, 'code': 30, 'lines': 34}}, {'./Cargo.toml': {'code': 14, 'lines': 20, 'comments': 2, 'blanks': 4}}],
    'YAML': [{'./mkdocs.yml': {'blanks': 6, 'lines': 69, 'code': 63, 'comments': 0}}],
    'Python': [{'./pytokei/__init__.py': {'comments': 0, 'code': 20, 'blanks': 1, 'lines': 21}}],
    'Makefile': [{'./Makefile': {'comments': 0, 'lines': 26, 'code': 18, 'blanks': 8}}],
    'Rust': [
        {'./src/lib.rs': {'blanks': 3, 'comments': 1, 'lines': 29, 'code': 25}},
        {'./src/pyconfig.rs': {'blanks': 12, 'lines': 67, 'code': 52, 'comments': 3}},
        {'./src/pysort.rs': {'code': 40, 'lines': 46, 'comments': 0, 'blanks': 6}},
        {'./src/pylanguage.rs': {'blanks': 22, 'lines': 129, 'comments': 2, 'code': 105}},
        {'./src/pystats.rs': {'blanks': 18, 'lines': 151, 'code': 130, 'comments': 3}},
        {'./src/pylanguages.rs': {'lines': 147, 'comments': 4, 'code': 124, 'blanks': 19}},
        {'./src/pylanguage_type.rs': {'comments': 23, 'code': 317, 'lines': 363, 'blanks': 23}}
    ],
    'Markdown': [{'./README.md': {'blanks': 21, 'comments': 25, 'code': 0, 'lines': 46}}]
}
```

By calling the previous method we get the reports associated to each *file* by *language type*.

## Accessing a single language

A `Languages` instance can be accessed by passing a `LanguageType` already parsed:

```python
>>> from pytokei import LanguageType
>>> rust = langs[LanguageType("Rust")]
```

Accessing the reports we can see what files are written in `Rust`:

```python
>>> print(rust.reports_plain())
[
    {'./src/lib.rs': {'comments': 1, 'blanks': 3, 'code': 25, 'lines': 29}},
    {'./src/pysort.rs': {'blanks': 6, 'code': 40, 'comments': 0, 'lines': 46}},
    {'./src/pylanguage.rs': {'comments': 2, 'lines': 129, 'blanks': 22, 'code': 105}},
    {'./src/pyconfig.rs': {'code': 52, 'lines': 67, 'blanks': 12, 'comments': 3}},
    {'./tests/data/rust.rs': {'comments': 2, 'code': 33, 'lines': 39, 'blanks': 4}},
    {'./src/pylanguages.rs': {'code': 124, 'lines': 147, 'blanks': 19, 'comments': 4}},
    {'./src/pystats.rs': {'lines': 151, 'blanks': 18, 'code': 130, 'comments': 3}},
    {'./src/pylanguage_type.rs': {'comments': 23, 'code': 317, 'lines': 363, 'blanks': 23}}
]
```

### Blobs of nested code

Sometimes nested languages can be found in a given file. If that is the case, it can be accessed in the `children` methods:

```python
>>> rust.children_plain()
{'Markdown': [{'./tests/data/rust.rs': {'lines': 3, 'code': 0, 'comments': 3, 'blanks': 0}}]}
```

In this case, there are docstrings inside a rust file:

```python
>>> rust.reports[4].stats.blobs
{LanguageType(Markdown): CodeStats(blanks: 0, code: 0, comments: 3, lines: 3)}
```

## Sorting

The reports of a given file can be sorted by one of the methods available:

```python
>>> pytokei.sort_types()
['Blanks', 'Comments', 'Code', 'Files', 'Lines']
```

Before and after sorting the files by the number of lines:

```python
>>> print(rust.reports)
[
    Report("./src/lib.rs"),
    Report("./src/pysort.rs"),
    Report("./src/pylanguage.rs"),
    Report("./src/pyconfig.rs"),
    Report("./tests/data/rust.rs"),
    Report("./src/pylanguages.rs"),
    Report("./src/pystats.rs"),
    Report("./src/pylanguage_type.rs")
]
>>> rust.sort_by(pytokei.Sort("lines"))
>>> print(rust.reports)
[
    Report("./src/pylanguage_type.rs"),
    Report("./src/pystats.rs"),
    Report("./src/pylanguages.rs"),
    Report("./src/pylanguage.rs"),
    Report("./src/pyconfig.rs"),
    Report("./src/pysort.rs"),
    Report("./tests/data/rust.rs"),
    Report("./src/lib.rs")
]
```


## Summary of a report

To see the sum of all the files found, the languages can be totaled up:

```python
>>> langs.total_plain()
{'code': 1291, 'lines': 1857, 'files': 34, 'comments': 273, 'blanks': 293}
```

To see all the functionalities exposed, please visit the. 
