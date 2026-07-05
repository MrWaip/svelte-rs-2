App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	function make() {
		return () => $.update(count);
	}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var event_handler = $.derived(make);
	$.template_effect(() => $.set_attribute(div, "title", $.get(count)));
	$.delegated("click", div, function(...$$args) {
		$.apply(() => $.get(event_handler), this, $$args, App, [5, 14], true, true);
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
$.delegate(["click"]);
