import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[2, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var a;
	var $$promises = $.run([async () => void (await $.track_reactivity_loss(Promise.resolve()))(), () => a = 0]);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.run_after_blockers([$$promises[1]], () => {
		$.bind_element_size(div, "clientWidth", function set($$value) {
			a = $$value;
		});
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
