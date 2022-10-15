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


class TestSort:
    @pytest.fixture
    def sort(self):
        return tokei.Sort()

    def test_repr(self, sort):
        assert repr(sort) == "Sort(Lines)"

    def test_from_str(self):
        sort = tokei.Sort.from_str("blanks")
        assert repr(sort) == "Sort(Blanks)"
        sort = tokei.Sort.from_str("Blanks")
        assert repr(sort) == "Sort(Blanks)"
        # TODO: Control for undefined errors from rust side
        # sort = tokei.PySort.from_str("undefined")
        # assert repr(sort) == "PySort(Blanks)"


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

    def test_content(self, stats):
        assert stats.content() == {"blanks": 0, "code": 0, "comments": 0, "lines": 0}

    def test_repr(self, stats):
        assert repr(stats) == "CodeStats(blanks: 0, code: 0, comments: 0, lines: 0)"


class TestReport:
    @pytest.fixture
    def report(self):
        return tokei.Report(str(pathlib.Path(SAMPLE_FILES_PATH) / "Dockerfile"))

    def test_name(self, report):
        assert report.name == str(pathlib.Path(SAMPLE_FILES_PATH) / "Dockerfile")

    def test_stats(self, report):
        assert isinstance(report.stats, tokei.CodeStats)


class TestLanguages:
    @pytest.fixture
    def languages(self):
        return tokei.Languages()

    def test_languages(self, languages):
        assert isinstance(languages, tokei.Languages)

    def test_names(self, languages):
        path = SAMPLE_FILES_PATH
        print(path)
        ignore = "nothing"  # This should take also a null string
        conf = tokei.Config()
        languages.get_statistics(path, ignore, conf)
        assert languages.language_names() == set(["Python", "Rust", "Dockerfile"])

    def test_languages_total(self):
        assert 1==2

    def test_languages_by_file(self):
        assert 1==2


class TestLanguageTypes:
    def test_language_types(self):
        lang_types = tokei.language_types()
        assert isinstance(lang_types, dict)

    def test_py_language(self):
        python = tokei.language_types()["Python"]
        assert isinstance(python, tokei.LanguageType)
        assert repr(python) == "LanguageType(Python)"


# TODO: Needs checks for possible errors.
