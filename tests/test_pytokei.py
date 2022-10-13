"""The tests correspond to a small subset of the
files found in https://github.com/XAMPPRocky/tokei/tree/master/tests/data
"""

import os
import pathlib
import pytokei as tokei
import pytest

here = os.path.dirname(os.path.abspath(__file__))
SAMPLE_FILES_PATH = str(pathlib.Path(here) / "data")


class TestPyConfig:
    @pytest.fixture
    def conf(self):
        return tokei.PyConfig()

    def test_columns(self, conf):
        assert conf.columns is None


class TestPySort:
    @pytest.fixture
    def sort(self):
        return tokei.PySort()

    def test_repr(self, sort):
        assert repr(sort) == "PySort(Lines)"

    def test_from_str(self):
        sort = tokei.PySort.from_str("blanks")
        assert repr(sort) == "PySort(Blanks)"
        sort = tokei.PySort.from_str("Blanks")
        assert repr(sort) == "PySort(Blanks)"
        # TODO: Control for undefined errors from rust side
        # sort = tokei.PySort.from_str("undefined")
        # assert repr(sort) == "PySort(Blanks)"


class TestPyCodeStats:
    @pytest.fixture
    def stats(self):
        return tokei.PyCodeStats()

    def test_blanks(self, stats):
        assert stats.blanks == 0

    def test_code(self, stats):
        assert stats.code == 0

    def test_comments(self, stats):
        assert stats.comments == 0

    def test_lines(self, stats):
        assert stats.lines() == 0

    def test_summarise(self, stats):
        assert isinstance(stats.summarise(), tokei.PyCodeStats)

    def test_content(self, stats):
        assert stats.content() == {"blanks": 0, "code": 0, "comments": 0, "lines": 0}


class TestPyLanguages:
    @pytest.fixture
    def languages(self):
        return tokei.PyLanguages()

    def test_languages(self, languages):
        assert isinstance(languages, tokei.PyLanguages)

    def test_names(self, languages):
        path = SAMPLE_FILES_PATH
        print(path)
        ignore = "nothing"  # This should take also a null string
        conf = tokei.PyConfig()
        languages.get_statistics(path, ignore, conf)
        assert languages.language_names() == set(["Python", "Rust", "Dockerfile"])
