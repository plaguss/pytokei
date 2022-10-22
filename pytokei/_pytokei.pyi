from typing import Optional

class Config:
    """A configuration struct for how Languages.get_statistics searches and counts languages.

    References
    ----------
    [tokei reference](https://docs.rs/tokei/latest/tokei/struct.Config.html)

    Examples
    --------
    >>> conf = pytokei.Config()
    >>> conf
    Config()
    """
    def __init__(self) -> None: ...
    @property
    def columns(self) -> int:
        """Width of columns to be printed to the terminal. This option is ignored in the library. 
        Default: *Auto detected width of the terminal*.
        [tokei reference](https://docs.rs/tokei/latest/tokei/struct.Config.html#structfield.columns).
        """
    @property
    def hidden(self) -> bool:
        """Count hidden files and directories.
        Default: *false*.
        [tokei reference](https://docs.rs/tokei/latest/tokei/struct.Config.html#structfield.hidden).
        """
    @property
    def no_ignore(self) -> bool:
        """Don't respect ignore files (.gitignore, .ignore, etc.).
        [tokei reference](https://docs.rs/tokei/latest/tokei/struct.Config.html#structfield.no_ignore).
        """
    @property
    def no_ignore_parent(self) -> bool:
        """Don't respect ignore files (.gitignore, .ignore, etc.) in parent directories.
        Default: *false*.
        [tokei reference](https://docs.rs/tokei/latest/tokei/struct.Config.html#structfield.no_ignore_parent).
        """
    @property
    def no_ignore_dot(self) -> bool:
        """Don't respect .ignore and .tokeignore files, including those in parent directories.
        Default: *false*.
        [tokei reference](https://docs.rs/tokei/latest/tokei/struct.Config.html#structfield.no_ignore_dot).
        """
    @property
    def no_ignore_vcs(self) -> bool:
        """Don't respect VCS ignore files (.gitignore, .hgignore, etc.), including those in parent directories.
        Default: *false*.
        [tokei reference](https://docs.rs/tokei/latest/tokei/struct.Config.html#structfield.no_ignore_vcs).
        """
    @property
    def treat_doc_strings_as_comments(self) -> bool:
        """Whether to treat doc strings in languages as comments.
        Default: *false*.
        [tokei reference](https://docs.rs/tokei/latest/tokei/struct.Config.html#structfield.treat_doc_strings_as_comments).
        """
    @staticmethod
    def from_config_files() -> Config:
        """Creates a Config from three configuration files if they are available.
        [tokei reference](https://docs.rs/tokei/latest/tokei/struct.Config.html#structfield.from_config_files).
        """
    def __repr__(self) -> str: ...

class Language:
    """A struct representing statistics about a single Language.

    References
    ----------
    [tokei reference](https://docs.rs/tokei/latest/tokei/struct.Language.html)

    Examples
    --------
    ```python
    >>> import pytokei
    >>> lang = pytokei.Language()
    >>> lang
    Language(empty: true)
    ```

    Note:
        `mark_inaccurate` is not defined, this may be better done from the rust side.
    """
    def __init__(self) -> None: ...
    @property
    def blanks(self) -> int:
        """The total number of blank lines.
        [tokei reference](https://docs.rs/tokei/latest/tokei/struct.Language.html#structfield.blanks).
        """
    @property
    def code(self) -> int:
        """The total number of lines of code.
        [tokei reference](https://docs.rs/tokei/latest/tokei/struct.Language.html#structfield.code).
        """
    @property
    def comments(self) -> int:
        """The total number of comments(both single, and multi-line).
        [tokei reference](https://docs.rs/tokei/latest/tokei/struct.Language.html#structfield.comments).
        """
    @property
    def reports(self) -> list[Report]:
        """A collection of statistics of individual files.
        [tokei reference](https://docs.rs/tokei/latest/tokei/struct.Language.html#structfield.reports).
        """
    def reports_plain(self) -> list[dict[str, dict[str, int]]]:
        """A version of `reports` method but in plain python objects.

        Instead of a list with reports, it returns a list with dicts containing
        as key the name of the file and a dict with each one of the reports's objects.
        """
    @property
    def children(self) -> dict[LanguageType, list[Report]]:
        """A map of any languages found in the reports.
        [tokei reference](https://docs.rs/tokei/latest/tokei/struct.Language.html#structfield.children).
        """
    def children_plain(self) -> dict[str, list[dict[str, dict[str, int]]]]:
        """The equivalent version of `reports_plain` method, but with children. """
    @property
    def innacurate(self) -> bool:
        """Whether this language had problems with file parsing.
        [tokei reference](https://docs.rs/tokei/latest/tokei/struct.Language.html#structfield.innacurate).
        """
    def lines(self) -> int:
        """Returns the total number of lines.
        """
    def add_report(self, report: Report) -> int:
        """Add a `Report` to the Language.
        This will not update the totals in the `Language` class.
        """
    def summarise(self) -> Language:
        """Creates a new `Language` from `self`, which is a summarised version
        of the language that doesn't contain any children.
        It will count non-blank lines in child languages as code unless the child
        language is considered "literate" then it will be counted as comments.
        """
    def total(self) -> None:
        """Totals up the statistics of the `Stat` class currently contained in the language. """
    def is_empty(self) -> bool:
        """Checks if the language is empty. Empty meaning it doesn't have any statistics. """
    def sort_by(self, category: Sort) -> None:
        """Sorts each of the `Report`s contained in the language based on what category is provided. """
    def files(self) -> int:
        """Counts the number of reports. """
    def __repr__(self) -> str: ...

class LanguageType:
    """Represents a individual programming language.

    Can be used to provide information about the language,
    such as multi line comments, single line comments,
    string literal syntax, whether a given language allows nesting comments.

    Note:
        This is defined as a struct in rust. There may be a better way
        of representing this object in python, but for the moment
        its a class which has the corresponding `LanguageType`
        represented underneath.

    Warning:
        The following methods aren't currently implemented:
        `from_path`, `from_file_extension`, `from_mime`,
        `from_shebang`, `parse`, `parse_from_str`, `parse_from_slice`. 

    Examples
    --------
    ```python
    >>> from pytokei import LanguageType
    >>> python = LanguageType("Python")
    >>> python
    LanguageType(Python)
    ```

    To see the languages defined, run the following:
    ```python
    >>> LanguageType.list()
    ['ABNF', 'ABAP', 'ActionScript', 'Ada', ...
    ```

    References
    ----------
    [tokei reference](https://docs.rs/tokei/latest/tokei/enum.LanguageType.html).
    The implementation of the different methods are here: [ref](https://docs.rs/tokei/latest/tokei/enum.LanguageType.html#impl)
    """
    def __init__(self) -> None: ...
    def __hash__(self) -> int: ...
    def __repr__(self) -> str: ...
    def name(self) -> str:
        """Returns the display name of a language. """
    # @staticmethod
    # def list() -> list[str]: ...  # How should this be typed??
    def is_literate(self) -> bool:
        """Returns whether the language is "literate", meaning that it
        considered to primarily be documentation and is counted primarily
        as comments rather than procedural code.
        """
    def line_comments(self) -> list[str]:
        """Returns the single line comments of a language.

        Examples
        --------
        ```python
        >>> from pytokei import LanguageType
        >>> python = LanguageType("Python")
        >>> python.line_comments()
        ['#']
        ```
        """
    def multi_line_comments(self) -> list[Optional[tuple[str]]]:
        """Returns the multi line comments of a language (if they have it).

        Examples
        --------
        ```python
        >>> from pytokei import LanguageType
        >>> python = LanguageType("Python")
        >>> python.multi_line_comments()
        []

        >>> rust = LanguageType("Rust")
        >>> rust.multi_line_comments()
        [('/*', '*/')]
        ```
        """
    def allows_nested(self) -> bool:
        """Returns whether the language allows nested multi line comments. """
    def nested_comments(self) -> list[tuple[str]]:
        """Returns what nested comments the language has. (Currently only D has any of this type.)

        Examples
        --------
        ```python
        >>> d = LanguageType("D")
        >>> d.nested_comments()
        [('/+', '+/')]
        ```
        """
    def quotes(self) -> list[tuple[str]]:
        """Returns the quotes of a language.

        Examples
        --------
        ```python
        >>> d = LanguageType("C")
        >>> d.quotes()
        [('"', '"')]
        ```
        """
    def verbatim_quotes(self) -> list[tuple[str]]:
        """Returns the verbatim quotes of a language.

        Examples
        --------
        ```python
        >>> LanguageType("C#").verbatim_quotes()
        [('@"', '"')]
        ```
        """
    def doc_quotes(self) -> list[tuple[str]]:
        """Returns the doc quotes of a language.

        Examples
        --------
        ```python
        >>> LanguageType("Python").doc_quotes()
        [('"""', '"""'), ("'''", "'''")]
        ```
        """
    def shebangs(self) -> list[str]:
        """Returns the shebang of a language.
        
        Examples
        --------
        ```python
        >>> LanguageType("BASH").shebangs()
        ['#!/bin/bash']
        ```
        """
    def important_syntax(self) -> list[str]:
        """Returns the parts of syntax that determines whether `tokei`
        can skip large parts of analysis.
        """

class Languages:
    """A class representing a list of languages counted in the provided directory.
    See [`LanguageType.list`](language_type.md)

    Examples
    --------
    ```python
    >>> from pytokei import Languages
    >>> langs = Languages()
    >>> langs
    Languages()
    ```

    References
    ----------
    [Languages implementation](https://docs.rs/tokei/latest/tokei/struct.Languages.html#impl)
    """
    def __init__(self) -> None: ...
    def get_statistics(
        self, paths: list[str], ignored: list[str], config: Config
    ) -> None:
        """Populates the Languages struct with statistics about languages provided by Language.

        Takes a list of of paths (as str) to recursively traverse, paths can be relative,
        absolute or glob paths.
        A second list of paths (as str) to ignore, these strings use the `.gitignore` syntax,
        such as `target` or `**/*.bk`.

        Parameters
        ----------
            paths : list[str]
                List of files to traverse. It may be a single directory.
            ignored : list[str]
                List of files to ignore. If you don't want anything ignored, just pass `["ignored"]`.
            config : Config
                Config instance. If you dont have any preferences, just pass `Config`.
        """
    def total(self) -> Language:
        """Summary of the Languages struct. """
    def language_names(self) -> Optional[list[str]]:
        """Returns the list of language names, if any was found. """
    def __getitem__(self, lang_type: LanguageType) -> Language | ValueError:
        """Implements the same functionality as in tokei to access the contents of a given
        languages object by a key.

        Corresponds to `let rust = &languages[&LanguageType::Rust];` in python
        """
    def get_languages(self) -> dict[LanguageType, Language]:
        """Exposes the inner struct from rust to the classes defined in python. """
    def files(self) -> dict[str, int]:
        """Total number of files in the value, corresponding to the language name key. """
    def get_languages_plain(self) -> dict[str, list[dict[str, dict[str, int]]]]:
        """The same method as `get_languages` but in python builtin objects. """

class Sort:
    """Used for sorting languages.
    
    Examples
    --------
    ```python
    >>> from pytokei import Sort
    >>> sort_method = Sort("Lines")
    >>> sort_method
    Sort(Lines)
    ```

    References
    ----------
    [tokei reference](https://docs.rs/tokei/latest/tokei/enum.Sort.html)
    """
    def __init__(self) -> None: ...
    @staticmethod
    def from_str(s: str) -> Sort | ValueError:
        """Another instantiation method from the name of the method.

        Examples
        --------
        ```python
        >>> from pytokei import Sort
        >>> sort_method = pytokei.Sort.from_str("lines")
        >>> sort_method
        Sort(Lines)
        ```
        sort = pytokei.Sort.from_str("lines")
        """
    def __repr__(self) -> str: ...

def sort_types() -> list[str]:
    """Helper function to obtain each variant of the `Sort` enum as a str.

    Examples
    --------
    ```python
    >>> from pytokei import sort_types
    >>> sort_types()
    ['Blanks', 'Comments', 'Code', 'Files', 'Lines']
    ```
    """

class CodeStats:
    """A class representing stats about a single blob of code.
    [tokei reference](https://docs.rs/tokei/latest/tokei/struct.CodeStats.html).

    Examples
    --------
    ```python
    >>> from pytokei import CodeStats
    >>> CodeStats()
    CodeStats(blanks: 0, code: 0, comments: 0, lines: 0)
    ```
    """
    def __init__(self) -> None: ...
    @property
    def blanks(self) -> int:
        """The blank lines in the blob.
        [tokei reference](https://docs.rs/tokei/latest/tokei/struct.CodeStats.html#structfield.blanks) 
        """
    @property
    def code(self) -> int:
        """The lines of code in the blob.
        [tokei reference](https://docs.rs/tokei/latest/tokei/struct.CodeStats.html#structfield.blanks) 
        """
    @property
    def comments(self) -> int:
        """The lines of comments in the blob.
        [tokei reference](https://docs.rs/tokei/latest/tokei/struct.CodeStats.html#structfield.comments)
        """
    @property
    def blobs(self) -> dict[LanguageType, CodeStats]:
        """Language blobs that were contained inside this blob, represented in the
        equivalent python objects.
        [tokei reference](https://docs.rs/tokei/latest/tokei/struct.CodeStats.html#structfield.blobs)
        """
    @property
    def blobs_plain(self) -> dict[str, dict[str, int]]:
        """Equivalent method to `blobs` but in builtin python objects. """
    def lines(self) -> int:
        """Get the total lines in a blob of code. """
    def summarise(self) -> CodeStats:
        """Creates a new `CodeStats` from an existing one with all of the child blobs merged. """
    def plain(self) -> dict[str, int]:
        """Returns the content of the blob as a dict, blanks, code, comments and lines. """
    def __repr__(self) -> str: ...

class Report:
    """A struct representing the statistics of a file.

    [tokei reference](https://docs.rs/tokei/latest/tokei/struct.Report.html).

    It can be constructed easily:

    Examples
    --------
    ```python
    >>> from pytokei import Report
    >>> Report("filename")
    Report("filename")
    ```

    But it isn't expected to be used like this, just get it from a parsed directory.
    """
    def __init__(self) -> None: ...
    @property
    def name(str) -> str:
        """Filename that represents. """
    @property
    def stats(str) -> CodeStats:
        """The code statistics found in the file. """
    def __repr__(self) -> str: ...
    def plain(self) -> dict[str, dict[str, int]]:
        """Representation of the object in builtin python objects, where the key corresponds 
        to the filename that generated it, and the value is the result of `CodeStats.plain` method.
        """
