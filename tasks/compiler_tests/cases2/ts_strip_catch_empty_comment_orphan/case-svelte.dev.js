import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[17, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let n = $.prop($$props, "n", 8);
	function run() {
		try {
			const a = 1;
			const b = 2;
			console.log(a, b);
		} catch {}
	}
	if (n()) {
		console.log(...$.log_if_contains_state("log", n()));
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, n()));
	$.delegated("click", button, run);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
