import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="file"/>`), App[$.FILENAME], [[2, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var a;
	var $$promises = $.run([async () => void (await $.track_reactivity_loss(Promise.resolve()))(), () => a = "a"]);
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.remove_input_defaults(input);
	$.run_after_blockers([$$promises[1]], () => {
		$.bind_value(input, function get() {
			return a;
		}, function set($$value) {
			a = $$value;
		});
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
