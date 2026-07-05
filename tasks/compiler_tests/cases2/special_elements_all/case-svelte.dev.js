App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<meta name="viewport" content="width=device-width"/>`), App[$.FILENAME], [[15, 4]]);
var root_1 = $.add_locations($.from_html(`<div><p> </p></div>`), App[$.FILENAME], [[
	22,
	0,
	[[23, 4]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	function handleEvent() {
		$.update(count);
	}
	function action(node) {
		return { destroy() {} };
	}
	var $$exports = { ...$.legacy_api() };
	var div = root_1();
	$.head("q2w0q4", ($$anchor) => {
		var meta = root();
		$.deferred_template_effect(() => {
			$.document.title = `App: ${$.get(count) ?? ""}`;
		});
		$.append($$anchor, meta);
	});
	$.event("scroll", $.window, handleEvent);
	$.event("visibilitychange", $.document, handleEvent);
	$.event("mouseenter", $.document.body, handleEvent);
	$.action($.document.body, ($$node) => action?.($$node));
	var p = $.child(div);
	var text = $.child(p);
	$.reset(p);
	$.reset(div);
	$.template_effect(() => $.set_text(text, `Count: ${$.get(count) ?? ""}`));
	$.append($$anchor, div);
	return $.pop($$exports);
}
