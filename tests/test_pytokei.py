"""The tests correspond to a small subset of the
files found in https://github.com/XAMPPRocky/tokei/tree/master/tests/data
"""

import os
import pathlib

import pytest

import pytokei as tokei

here = os.path.dirname(os.path.abspath(__file__))
SAMPLE_FILES_PATH = str(pathlib.Path(here) / "data")


class TestConfig:
    @pytest.fixture
    def conf(self):
        return tokei.Config()

    def test_columns(self, conf):
        assert conf.columns is None

    def test_repr(self, conf):
        assert repr(conf) == "Config()"

    def test_read_from_file(self, conf):
        conf = tokei.Config.from_config_files()
        assert isinstance(conf, tokei.Config)


class TestSort:
    @pytest.fixture
    def sort(self):
        return tokei.Sort("lines")

    def test_repr(self, sort):
        assert repr(sort) == "Sort(Lines)"

    def test_from_str(self):
        sort = tokei.Sort.from_str("blanks")
        assert repr(sort) == "Sort(Blanks)"
        sort = tokei.Sort.from_str("Blanks")
        assert repr(sort) == "Sort(Blanks)"
        with pytest.raises(ValueError):
            tokei.Sort.from_str("undefined")


def test_sort_types():
    assert tokei.sort_types() == ["Blanks", "Comments", "Code", "Files", "Lines"]


class TestCodeStats:
    @pytest.fixture
    def stats(self):
        return tokei.CodeStats()

    def test_blanks(self, stats):
        assert stats.blanks == 0

    def test_code(self, stats):
        assert stats.code == 0

    def test_comments(self, stats):
        assert stats.comments == 0

    def test_lines(self, stats):
        assert stats.lines() == 0

    def test_summarise(self, stats):
        assert isinstance(stats.summarise(), tokei.CodeStats)

    def test_stats_plain(self, stats):
        assert stats.plain() == {"blanks": 0, "code": 0, "comments": 0, "lines": 0}

    def test_repr(self, stats):
        assert repr(stats) == "CodeStats(blanks: 0, code: 0, comments: 0, lines: 0)"

    def test_blobs(self, stats):
        blobs = stats.blobs
        assert isinstance(blobs, dict)
        assert len(blobs) == 0  # Without anything parsed has no content

    def test_blobs_plain(self, stats):
        blobs = stats.blobs_plain()
        assert isinstance(blobs, dict)
        assert len(blobs) == 0  # Without anything parsed has no content


class TestReport:
    @pytest.fixture
    def report(self):
        return tokei.Report(str(pathlib.Path(SAMPLE_FILES_PATH) / "Dockerfile"))

    def test_name(self, report):
        assert report.name == str(pathlib.Path(SAMPLE_FILES_PATH) / "Dockerfile")

    def test_stats(self, report):
        assert isinstance(report.stats, tokei.CodeStats)

    def test_repr(self, report):
        # Test only that expected strings are contained
        # in the name to avoid writing the full path it shows
        repr_str = repr(report)
        assert repr_str.startswith("Report(")
        assert repr_str.endswith('Dockerfile")')

    def test_report_plain(self, report):
        plain = report.plain()
        assert isinstance(plain, dict)
        filename = list(plain.keys())[0]
        assert filename.endswith("Dockerfile")
        assert plain[filename] == {"blanks": 0, "code": 0, "comments": 0, "lines": 0}


class TestLanguages:
    @pytest.fixture
    def languages(self):
        return tokei.Languages()

    def test_languages(self, languages):
        assert isinstance(languages, tokei.Languages)

    def test_names(self, languages):
        path = SAMPLE_FILES_PATH
        ignore = "nothing"  # This should take also a null string
        conf = tokei.Config()
        languages.get_statistics([path], [ignore], conf)
        assert languages.language_names() == ["Dockerfile", "Python", "Rust", "TOML"]

    def test_get_statistics_multipath(self):
        langs = tokei.Languages()
        path1 = str(pathlib.Path(here) / "data" / "python1.py")
        path2 = str(pathlib.Path(here) / "data" / "Dockerfile")
        langs.get_statistics([path1, path2], ["ignored"], tokei.Config())


class TestLanguageType:
    def test_language_types(self):
        lang_types = tokei.LanguageType.list()
        assert isinstance(lang_types, list)

    @pytest.fixture
    def rust_lang(self):
        return tokei.LanguageType("Rust")

    def test_repr(self):
        python = tokei.LanguageType("Python")
        assert repr(python) == "LanguageType(Python)"

    def test_name(self):
        assert "Python" == tokei.LanguageType("Python").name()

    def test_for_error(self):
        with pytest.raises(ValueError):
            tokei.LanguageType("inexistent")

    def test_is_literate(self, rust_lang):
        assert rust_lang.is_literate() is False

    def test_line_comments(self, rust_lang):
        assert rust_lang.line_comments() == ["//"]
        assert tokei.LanguageType("Abap").line_comments() == ["*", '"']

    def test_multiline_comments(self, rust_lang):
        assert rust_lang.multi_line_comments() == [("/*", "*/")]
        assert tokei.LanguageType("ASP.NET").multi_line_comments() == [
            (
                "<!--",
                "-->",
            ),
            (
                "<%--",
                "-->",
            ),
        ]

    def test_allows_nested(self, rust_lang):
        assert rust_lang.allows_nested() is True

    def test_nested_comments(self):
        assert tokei.LanguageType("D").nested_comments() == [("/+", "+/")]

    def test_quotes(self):
        assert tokei.LanguageType("C").quotes() == [('"', '"')]

    def test_verbatim_quotes(self):
        assert tokei.LanguageType("C#").verbatim_quotes() == [('@"', '"')]

    def test_doc_quotes(self):
        assert tokei.LanguageType("Python").doc_quotes() == [
            ('"""', '"""'),
            ("'''", "'''"),
        ]

    def test_shebangs(self):
        tokei.LanguageType("BASH").shebangs() == ["#!/bin/bash"]

    def test_important_syntax(self, rust_lang):
        assert rust_lang.important_syntax() == ['#"', '"', "/*", "///", "//!"]

    @pytest.mark.skip
    def test_from_path(self):
        raise NotImplementedError

    @pytest.mark.skip
    def test_from_file_extension(self, rust_lang):
        raise NotImplementedError

    @pytest.mark.skip
    def test_from_mime(self, rust_lang):
        raise NotImplementedError

    @pytest.mark.skip
    def test_from_shebang(self, rust_lang):
        raise NotImplementedError


class TestLanguage:
    @pytest.fixture
    def language(self):
        return tokei.Language()

    def test_blanks(self, language):
        assert language.blanks == 0

    def test_code(self, language):
        assert language.code == 0

    def test_comments(self, language):
        assert language.comments == 0

    def test_reports(self, language):
        assert language.reports == []

    def test_children(self, language):
        assert language.children == {}

    def test_inaccurate(self, language):
        assert language.inaccurate == 0

    def test_lines(self, language):
        assert language.lines() == 0

    def test_add_report(self):
        langs = tokei.Language()
        assert len(langs.reports) == 0
        langs.add_report(tokei.Report("something"))
        assert len(langs.reports) == 1

    def test_summarise(self, language):
        assert isinstance(language.summarise(), tokei.Language)

    def test_is_empty(self, language):
        assert language.is_empty() is True

    def test_repr(self, language):
        assert repr(language) == "Language(empty: true)"

    def test_files(self, language):
        assert language.files() == 0


# TODO: Needs checks for possible errors.

# These should be considered integration tests, the code
# to be run on real data for proper testing.


class TestPytokei:
    # Test for errors on paths
    @pytest.fixture
    def languages(self) -> tokei.Languages:
        langs = tokei.Languages()
        langs.get_statistics([str(SAMPLE_FILES_PATH)], ["ignored"], tokei.Config())
        return langs

    def test_access_parsed_languages(self, languages):
        langs = languages.language_names()
        assert langs == ["Dockerfile", "Python", "Rust", "TOML"]

    def test_languages_getitem(self, languages):
        lang = languages[tokei.LanguageType("Dockerfile")]
        assert isinstance(lang, tokei.Language)
        with pytest.raises(ValueError):
            languages[tokei.LanguageType("Java")]

    def test_get_languages(self, languages):
        inner = languages.get_languages()
        assert isinstance(inner, dict)
        assert isinstance(list(inner.keys())[0], tokei.LanguageType)
        assert isinstance(list(inner.values())[0], tokei.Language)
        assert len(inner) == 4
        assert all([~v.is_empty() for v in inner.values()])

    def test_language_total(self, languages):
        assert len(languages.files()) == 4
        lang = languages.total()
        assert lang.blanks == 18
        assert lang.code == 66
        assert lang.comments == 13
        assert lang.lines() == 97

    def test_language_sort_by(self, languages):
        lang = languages[tokei.LanguageType("Python")]
        # assert lang.reports[0].name.endswith("python1.py")
        lang.sort_by(tokei.Sort("Lines"))
        assert lang.reports[0].name.endswith("python2.py")

    def test_language_get_reports(self, languages):
        reports = languages.get_languages()[tokei.LanguageType("Python")].reports
        assert isinstance(reports, list)
        assert all(isinstance(r, tokei.Report) for r in reports)

    def test_language_get_reports_plain(self, languages):
        reports_plain = languages.get_languages()[
            tokei.LanguageType("Rust")
        ].reports_plain()
        assert isinstance(reports_plain, list)
        assert all([isinstance(r, dict) for r in reports_plain])
        assert len(reports_plain) == 1
        filename = list(reports_plain[0].keys())[0]
        assert reports_plain[0][filename] == {
            "lines": 39,
            "blanks": 4,
            "code": 33,
            "comments": 2,
        }

    def test_language_get_children(self, languages):
        lang = languages.total()
        children = lang.children
        assert len(children) == 4
        assert all(isinstance(k, tokei.LanguageType) for k in children.keys())
        reports = children[tokei.LanguageType("Python")]
        assert len(reports) == 2
        assert all(isinstance(r, tokei.Report) for r in reports)

    def test_language_get_children_plain(self, languages):
        lang = languages.total()
        children_plain = lang.children_plain()
        assert len(children_plain) == 4
        toml_report = children_plain["TOML"][0]
        filename = list(toml_report.keys())[0]
        assert pathlib.Path(filename).name == "tokei.example.toml"
        assert toml_report[filename] == {
            "blanks": 0,
            "lines": 8,
            "code": 4,
            "comments": 4,
        }

    def test_languages_files(self, languages):
        assert languages.files() == {"Dockerfile": 1, "Python": 2, "Rust": 1, "TOML": 1}

    def test_languages_get_languages_plain(self, languages):
        received = languages.get_languages_plain()
        assert isinstance(received, dict)
        assert len(received) == 4
        assert isinstance(received["TOML"], list)
        assert len(received["TOML"]) == 1
        assert list(received["TOML"][0].keys())[0].endswith("tokei.example.toml")

    def test_languages_total_plain(self, languages):
        totals = languages.total_plain()
        columns = ["blanks", "code", "comments", "files", "lines"]
        values = [18, 66, 13, 5, 97]
        assert all([totals[c] == v for c, v in zip(columns, values)])

    def test_languages_default_report_plain(self, languages):
        report = languages.report_compact_plain()
        lang_names = ["TOML", "Dockerfile", "Python", "Rust"]
        stats = ["files", "lines", "code", "comments", "blanks"]
        values_dockerfile = [1, 16, 7, 3, 6]
        assert all([name in report.keys() for name in lang_names])
        assert all([report["Dockerfile"][k] == v for k, v in zip(stats, values_dockerfile)])
