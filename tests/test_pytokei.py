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
    def test_one(self):
        pass
