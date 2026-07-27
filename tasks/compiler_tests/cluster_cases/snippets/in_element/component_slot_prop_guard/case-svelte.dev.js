App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<b>hi</b>`), App[$.FILENAME], [[1, 27]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	{
		const foo = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			var b = root();
			$.append($$anchor, b);
		});
		$.add_svelte_meta(() => Component($$anchor, {
			foo,
			$$slots: { foo: true }
		}), "component", App, 1, 0, { componentTag: "Component" });
	}
	return $.pop($$exports);
}
