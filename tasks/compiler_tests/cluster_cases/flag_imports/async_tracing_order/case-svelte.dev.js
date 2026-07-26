import "svelte/internal/flags/tracing";
import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	function read() {
		return $.trace(() => "read ((unknown):4:1)", () => {
			return $.get(count);
		});
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(($0) => $.set_text(text, $0), [() => read()]);
	$.delegated("click", button, function click() {
		return $.update(count);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
