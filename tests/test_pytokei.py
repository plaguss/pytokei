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

    @pytest.mark.skip
    def test_repr(self, conf):
        pass

    @pytest.mark.skip
    def test_read_from_file(self, conf):
        pass


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
        languages.get_statistics(path, ignore, conf)
        assert languages.language_names() == set(["Python", "Rust", "Dockerfile", "TOML"])

    @pytest.mark.skip
    def test_languages_by_file(self):
        pass


class TestLanguageType:
    def test_language_types(self):
        lang_types = tokei.language_types()
        assert isinstance(lang_types, dict)

    def test_py_language(self):
        python = tokei.language_types()["Python"]
        assert isinstance(python, tokei.LanguageType)

    def test_repr(self):
        python = tokei.language_types()["Python"]
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

# These should be considered integration tests, the code
# to be run on real data for proper testing.


class TestPytokei:
    # Test for errors on paths
    @pytest.fixture
    def languages(self):
        langs = tokei.Languages()
        langs.get_statistics(str(SAMPLE_FILES_PATH), "ignored", tokei.Config())
        return langs

    def test_access_parsed_languages(self, languages):
        langs = languages.language_names()
        assert langs == {"Dockerfile", "Python", "Rust", "TOML"}

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

    def test_language_total(self, languages):
        lang = languages.total()
        assert isinstance(lang, tokei.Language)

    @pytest.mark.skip
    def test_language_sort_by(self, languages):
        pass

    @pytest.mark.skip
    def test_language_get_reports_plain(self):
        pass

    @pytest.mark.skip
    def test_language_get_children_plain(self):
        pass

    @pytest.mark.skip
    def test_blobs_from_code_stats(self):
        blobs = 1
        # TODO: If it fails, do this check once parsed
        assert isinstance(list(blobs.keys())[0], tokei.LanguageType)
        assert isinstance(list(blobs.values())[0], tokei.CodeStats)

    @pytest.mark.skip
    def test_blobs_plain_from_code_stats(self):
        blobs = 1
        # TODO: If it fails, do this check once parsed
        assert isinstance(list(blobs.keys())[0], tokei.LanguageType)
        assert isinstance(list(blobs.values())[0], tokei.CodeStats)
