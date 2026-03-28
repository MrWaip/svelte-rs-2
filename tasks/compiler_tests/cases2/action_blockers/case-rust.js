import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>hello</div>`);
export default function App($$anchor) {
	function tooltip(node, arg) {}
	var data, config;
	var $$promises = $.run([async () => data = await fetch("/api"), () => config = $.proxy(data.config)]);
	var div = root();
	$.run_after_blockers([$$promises[1]], () => {
		$.action(div, ($$node, $$action_arg) => tooltip?.($$node, $$action_arg), () => config);
	});
	$.append($$anchor, div);
}
