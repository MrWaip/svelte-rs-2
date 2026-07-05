App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>+</button>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const row = $.wrap_snippet(App, function($$anchor) {
		$.validate_snippet_args(...arguments);
		var button = root();
		$.template_effect(() => {
			console.log({ count: $.snapshot($.get(count)) });
			debugger;
		});
		$.delegated("click", button, function click() {
			return $.update(count);
		});
		$.append($$anchor, button);
	});
	let count = $.tag($.state(0), "count");
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => row($$anchor), "render", App, 10, 0);
	return $.pop($$exports);
}
$.delegate(["click"]);
