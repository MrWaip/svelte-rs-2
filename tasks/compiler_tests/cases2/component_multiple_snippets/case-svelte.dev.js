App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<h1>Header</h1>`), App[$.FILENAME], [[3, 2]]);
var root_1 = $.add_locations($.from_html(`<p>Footer</p>`), App[$.FILENAME], [[6, 2]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	{
		const header = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			var h1 = root();
			$.append($$anchor, h1);
		});
		const footer = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			var p = root_1();
			$.append($$anchor, p);
		});
		$.add_svelte_meta(() => Card($$anchor, {
			header,
			footer,
			$$slots: {
				header: true,
				footer: true
			}
		}), "component", App, 1, 0, { componentTag: "Card" });
	}
	return $.pop($$exports);
}
