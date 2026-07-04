import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { numbers } from "./data.js";
var $$_import_numbers = $.reactive_import(() => numbers);
var root = $.add_locations($.from_html(`<p> </p> <button>add</button>`, 1), App[$.FILENAME], [[9, 0], [10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	function add() {
		$$_import_numbers($$_import_numbers()[$$_import_numbers().length] = $$_import_numbers().length + 1);
	}
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var button = $.sibling(p, 2);
	$.template_effect(($0) => $.set_text(text, $0), [() => ($.deep_read_state($$_import_numbers()), $.untrack(() => $$_import_numbers().join(" + ")))]);
	$.event("click", button, add);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
