import "svelte/internal/flags/tracing";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	$.user_effect(() => {
		return $.trace(() => "$effect(...) ((unknown):3:9)", () => {
			$.get(count);
		});
	});
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.delegated("click", button, function click() {
		return $.update(count);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
