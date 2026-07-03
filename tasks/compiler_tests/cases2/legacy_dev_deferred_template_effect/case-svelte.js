import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>+</button>`), App[$.FILENAME], [[13, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let count = $.tag($.mutable_source(0), "count");
	function bump() {
		$.set(count, $.get(count) + 1);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.head("q2w0q4", ($$anchor) => {
		$.deferred_template_effect(() => {
			$.document.title = `Count: ${$.get(count) ?? ""}`;
		});
	});
	$.delegated("click", button, bump);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
