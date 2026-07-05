App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<h2></h2>`), App[$.FILENAME], [[7, 2]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = 0;
	var $$exports = { ...$.legacy_api() };
	{
		const header = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			var h2 = root();
			h2.textContent = "Title 0";
			$.append($$anchor, h2);
		});
		$.add_svelte_meta(() => Dialog($$anchor, {
			header,
			$$slots: { header: true }
		}), "component", App, 5, 0, { componentTag: "Dialog" });
	}
	return $.pop($$exports);
}
