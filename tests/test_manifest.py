"""The dialogs in plugin.json: only what SolidCraft 0.43 can draw, and every choice with a title."""
import json

from helpers import *

DRAWN = {"type", "title", "description", "default", "minimum", "maximum", "minLength", "maxLength", "oneOf"}


def test_choices_have_titles(approval):
    manifest = json.loads((ROOT / "plugin.json").read_text(encoding="utf-8"))
    choices = 0
    for entry in manifest["runtime"]["entries"]:
        for name, field in entry["input_schema"]["properties"].items():
            at = (entry["name"], name)
            # A keyword the dialog cannot draw removes the whole entry from the menu.
            assert set(field) <= DRAWN, (at, set(field) - DRAWN)
            if "oneOf" not in field:
                continue
            choices += 1
            options = field["oneOf"]
            values = [o["const"] for o in options]
            assert field["type"] == "string" and options and len(set(values)) == len(values), at
            assert field["default"] in values, at
            for o in options:
                assert set(o) <= {"const", "title", "description"} and isinstance(o["const"], str) and o["title"], at
    assert choices == 22, choices
    # The plugin still gets the value, not the title.
    results = cli([plugin(approval, "corner", {"shape": "diagonal", "doors": "hinged right"}), MEASURE])
    assert "Cabinet Door" in boxes(results[1]), boxes(results[1])
