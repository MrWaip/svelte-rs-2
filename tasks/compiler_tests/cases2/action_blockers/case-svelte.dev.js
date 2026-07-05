import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>hello</div>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function tooltip(node, arg) {}
	var data, config;
	var $$promises = $.run([async () => data = (await $.track_reactivity_loss(fetch("/api")))(), () => config = $.tag_proxy($.proxy(data.config), "config")]);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.run_after_blockers([$$promises[1]], () => {
		$.action(div, ($$node, $$action_arg) => tooltip?.($$node, $$action_arg), () => config);
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
