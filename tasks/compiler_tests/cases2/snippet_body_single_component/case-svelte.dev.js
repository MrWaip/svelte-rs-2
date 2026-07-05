App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	{
		const right = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			$.add_svelte_meta(() => Btn($$anchor, {}), "component", App, 3, 2, { componentTag: "Btn" });
		});
		$.add_svelte_meta(() => Header($$anchor, {
			right,
			$$slots: { right: true }
		}), "component", App, 1, 0, { componentTag: "Header" });
	}
	return $.pop($$exports);
}
