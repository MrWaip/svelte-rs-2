App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<span>hi</span>`), App[$.FILENAME], [[7, 8]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	{
		const extra_element = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			var span = root();
			$.append($$anchor, span);
		});
		$.add_svelte_meta(() => Child($$anchor, {
			extra_element,
			$$slots: { extra_element: true }
		}), "component", App, 5, 0, { componentTag: "Child" });
	}
	return $.pop($$exports);
}
