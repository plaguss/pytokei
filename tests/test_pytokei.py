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

    @pytest.mark.skip
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
        ignore = "nothing"  # This should take also a null string
        conf = tokei.Config()
        languages.get_statistics(path, ignore, conf)
        assert languages.language_names() == set(["Python", "Rust", "Dockerfile"])


class TestLanguageType:
    def test_language_types(self):
        lang_types = tokei.language_types()
        assert isinstance(lang_types, dict)

    def test_py_language(self):
        python = tokei.language_types()["Python"]
        assert isinstance(python, tokei.LanguageType)
        assert repr(python) == "LanguageType(Python)"


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

# TODO: Needs checks for possible errors.

# These should be considered integration tests, need the 
# code to be run on real data for proper testing.

class TestPytokei:
    # Test for errors on paths
    @pytest.fixture
    def languages(self):
        langs = tokei.Languages()
        langs.get_statistics(str(SAMPLE_FILES_PATH), "ignored", tokei.Config())
        return langs

    def test_access_parsed_languages(self, languages):
        langs = languages.language_names()
        assert langs == {'Dockerfile', 'Python', 'Rust'}

    def test_languages_getattr(self, languages):
        lang = languages[tokei.LanguageType("Dockerfile")]
        assert isinstance(lang, tokei.Language)
        with pytest.raises(ValueError):
            languages[tokei.LanguageType("Java")]

    @pytest.mark.skip
    def test_languages_by_file(self):
        pass

    @pytest.mark.skip
    def test_languages_summary(self):
        pass

    @pytest.mark.skip
    def test_language_total(self):
        pass

    @pytest.mark.skip
    def test_language_sort_by(self):
        pass
