"""Check the local publishing target without contacting a package registry."""

import os
from pathlib import Path
import subprocess
import tempfile
import unittest


MAKEFILE = Path(__file__).resolve().parents[1] / "Makefile"


class PublishTests(unittest.TestCase):
    def test_publish_finds_the_newly_built_wheel(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            binaries = root / "bin"
            package = root / "package"
            binaries.mkdir()
            package.mkdir()
            record = root / "upload"
            commands = {
                "maturin": "mkdir -p dist\ntouch dist/pygraphina-0.0.0-py3-none-any.whl\n",
                "uv": 'printf "%s\\n" "$@" "$UV_PUBLISH_TOKEN" > "$PUBLISH_RECORD"\n',
            }
            for name, command in commands.items():
                executable = binaries / name
                executable.write_text("#!/bin/sh\nset -eu\n" + command)
                executable.chmod(0o755)
            result = subprocess.run(
                ["make", "-f", str(MAKEFILE), "publish-py", f"PYGRAPHINA_DIR={package}"],
                env={
                    **os.environ,
                    "PATH": f"{binaries}:{os.environ['PATH']}",
                    "PYPI_TOKEN": "test-token",
                    "PUBLISH_RECORD": str(record),
                },
                capture_output=True,
                text=True,
                check=False,
            )
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertEqual(
                record.read_text().splitlines(),
                ["publish", str(package / "dist/pygraphina-0.0.0-py3-none-any.whl"), "test-token"],
            )


if __name__ == "__main__":
    unittest.main()
