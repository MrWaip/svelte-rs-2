App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const row = $.wrap_snippet(App, function($$anchor) {
		$.validate_snippet_args(...arguments);
		const kLit = $.tag($.derived(() => "x"), "kLit");
		$.get(kLit);
		const kArith = $.tag($.derived(() => plain + 1), "kArith");
		$.get(kArith);
		$.add_svelte_meta(() => Child($$anchor, {
			kLit: $.get(kLit),
			kArith: $.get(kArith)
		}), "component", App, 9, 1, { componentTag: "Child" });
	});
	let plain = 7;
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => row($$anchor), "render", App, 12, 0);
	return $.pop($$exports);
}
