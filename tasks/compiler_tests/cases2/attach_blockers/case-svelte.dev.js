import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>hello</div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var data, handler;
	var $$promises = $.run([async () => data = (await $.track_reactivity_loss(fetch("/api")))(), () => handler = $.tag_proxy($.proxy(data.handler), "handler")]);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.run_after_blockers([$$promises[1]], () => {
		$.attach(div, () => handler);
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
