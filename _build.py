from __future__ import annotations

import contextlib
import ctypes
import os
import platform
import re
import subprocess

from maturin import (  # noqa: F401
    build_editable as _maturin_build_editable,
    build_sdist as build_sdist,
    build_wheel as _maturin_build_wheel,
    get_requires_for_build_sdist as get_requires_for_build_sdist,
    get_requires_for_build_wheel as get_requires_for_build_wheel,
    prepare_metadata_for_build_wheel as _maturin_prepare_metadata_for_build_wheel,
)

_PYPROJECT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "pyproject.toml")

_POLARS_RE = re.compile(r"""(?P<q>["'])polars(?=[<>=!~;\[\s"'])""")


def _x86() -> bool:
    return platform.machine().lower() in {"x86_64", "amd64", "x64", "i386", "i686"}


def _should_use_polars_lts_cpu() -> bool:
    if not _x86():
        return False

    system = platform.system()
    try:
        if system == "Linux":
            with open("/proc/cpuinfo", encoding="utf-8") as f:
                cpuinfo = f.read()
            return not bool(re.search(r"\bavx2\b", cpuinfo))
        elif system == "Darwin":
            out = subprocess.run(
                ["sysctl", "-n", "machdep.cpu.leaf7_features"],
                capture_output=True,
                text=True,
                check=False,
            ).stdout
            return "avx2" not in out.lower()
        elif system == "Windows":
            return not bool(ctypes.windll.kernel32.IsProcessorFeaturePresent(40))
    except Exception:
        return False
    return False


@contextlib.contextmanager
def _monkey_patch_deps():
    if not _should_use_polars_lts_cpu():
        yield
        return

    with open(_PYPROJECT, encoding="utf-8") as f:
        original = f.read()
    patched = _POLARS_RE.sub(r"\g<q>polars-lts-cpu", original)
    if patched == original:
        # Nothing to swap (already lts, or polars not pinned here).
        yield
        return

    with open(_PYPROJECT, "w", encoding="utf-8") as f:
        f.write(patched)
    try:
        yield
    finally:
        with open(_PYPROJECT, "w", encoding="utf-8") as f:
            f.write(original)


def build_wheel(wheel_directory, config_settings=None, metadata_directory=None):
    with _monkey_patch_deps():
        return _maturin_build_wheel(wheel_directory, config_settings, metadata_directory)


def build_editable(wheel_directory, config_settings=None, metadata_directory=None):
    with _monkey_patch_deps():
        return _maturin_build_editable(wheel_directory, config_settings, metadata_directory)


def prepare_metadata_for_build_wheel(metadata_directory, config_settings=None):
    with _monkey_patch_deps():
        return _maturin_prepare_metadata_for_build_wheel(metadata_directory, config_settings)
