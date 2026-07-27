import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<select><option>x</option></select>`), App[$.FILENAME], [[
	2,
	0,
	[[2, 23]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var a;
	var $$promises = $.run([async () => void (await $.track_reactivity_loss(Promise.resolve()))(), () => a = "a"]);
	var $$exports = { ...$.legacy_api() };
	var select = root();
	$.run_after_blockers([$$promises[1]], () => {
		$.bind_select_value(select, function get() {
			return a;
		}, function set($$value) {
			a = $$value;
		});
	});
	$.append($$anchor, select);
	return $.pop($$exports);
}
