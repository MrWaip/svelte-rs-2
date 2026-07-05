import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = 1;
	var data, value;
	var $$promises = $.run([async () => data = (await $.track_reactivity_loss(fetch("/api")))(), () => value = $.tag($.state($.proxy(data.text)), "value")]);
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.remove_input_defaults(input);
	$.run_after_blockers([$$promises[1]], () => {
		$.bind_value(input, function get() {
			return $.get(value);
		}, function set($$value) {
			$.set(value, $$value);
		});
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
