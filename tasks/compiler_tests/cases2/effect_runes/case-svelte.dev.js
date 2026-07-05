App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[13, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	$.set(count, $.get(count) + 1);
	$.user_effect(() => {
		console.log(...$.log_if_contains_state("log", "count:", $.get(count)));
	});
	$.user_pre_effect(() => {
		console.log(...$.log_if_contains_state("log", "pre-effect:", $.get(count)));
	});
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.append($$anchor, p);
	return $.pop($$exports);
}
