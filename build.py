"""Build, package and end-to-end test the plugin.

    python build.py            build dist/woodwork/ and dist/woodwork-<version>.zip, then test
    python build.py --install  also install the zip into SolidCraft
    python build.py corner     only the end-to-end tests with "corner" in their file or name

The end-to-end tests are in tests/, one file per function of the plugin.

Needs: cargo (with the wasm32-unknown-unknown target), wasm-tools, solidcraft-cli.
"""
import json
import os
import shutil
import subprocess
import sys
import zipfile
from pathlib import Path

ROOT = Path(__file__).parent
PACKAGE_ID = json.loads((ROOT / "plugin.json").read_text(encoding="utf-8"))["id"]
PKG = ROOT / "dist" / PACKAGE_ID
os.environ["PATH"] += os.pathsep + str(Path.home() / ".cargo" / "bin")
CLI = shutil.which("solidcraft-cli") or r"C:\Program Files\SolidCraft\solidcraft-cli.exe"


def sh(*args):
    subprocess.run(args, cwd=ROOT, check=True)


def cli(script, *extra):
    """Run a command script through solidcraft-cli and return the results."""
    path = ROOT / "dist" / "script.json"
    path.write_text(json.dumps(script), encoding="utf-8")
    out = ROOT / "dist" / "results.json"
    sh(CLI, "run", str(path), "--results", "--quiet", "--json-out", str(out), *extra)
    return json.loads(out.read_text(encoding="utf-8"))


def plugin(approval, entry, params=None):
    return {"command": "plugin.run", "params": {
        "path": str(PKG), "approval": approval, "entry": entry, "params": params or {}}}


def build():
    sh("cargo", "test", "--quiet")
    sh("cargo", "build", "--release", "--target", "wasm32-unknown-unknown")
    PKG.mkdir(parents=True, exist_ok=True)
    target = Path(os.environ.get("CARGO_TARGET_DIR", ROOT / "target"))
    sh("wasm-tools", "component", "new",
       str(target / "wasm32-unknown-unknown/release/woodwork.wasm"), "-o", str(PKG / "woodwork.wasm"))
    # SolidCraft caps manifest bytes; keep the readable source, compact the package.
    manifest = json.loads((ROOT / "plugin.json").read_text(encoding="utf-8"))
    (PKG / "plugin.json").write_text(json.dumps(manifest, separators=(",", ":")) + "\n", encoding="utf-8")
    shutil.copy(ROOT / "LICENSE", PKG)
    version = json.loads((PKG / "plugin.json").read_text(encoding="utf-8"))["version"]
    archive = ROOT / "dist" / f"{PACKAGE_ID}-{version}.zip"
    with zipfile.ZipFile(archive, "w", zipfile.ZIP_DEFLATED) as z:
        for name in ("plugin.json", "woodwork.wasm", "LICENSE"):
            z.write(PKG / name, f"{PACKAGE_ID}/{name}")
    return archive


def test(only=()):
    """Run the end-to-end tests in tests/test_*.py: every `test_*(approval)` function, or those
    whose file or name contains one of the words in `only`."""
    import importlib
    sys.path[:0] = [str(ROOT), str(ROOT / "tests")]
    approval = cli([{"command": "plugin.inspect", "params": {"path": str(PKG)}}])[0]["approval"]
    ran = 0
    for path in sorted((ROOT / "tests").glob("test_*.py")):
        module = importlib.import_module(path.stem)
        for name, function in vars(module).items():
            label = f"{path.stem}.{name}"
            if name.startswith("test_") and (not only or any(word in label for word in only)):
                function(approval)
                print("ok  ", label, flush=True)
                ran += 1
    assert ran, f"no test matches {only}"
    print("PASS" if not only else f"PASS ({ran} of the tests)")


if __name__ == "__main__":
    archive = build()
    test([word for word in sys.argv[1:] if not word.startswith("-")])
    if "--install" in sys.argv:
        print(cli([{"command": "plugin.install", "params": {"path": str(archive), "replace": True}}]))
