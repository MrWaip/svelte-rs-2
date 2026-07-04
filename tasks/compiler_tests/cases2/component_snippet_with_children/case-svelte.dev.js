App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<h2>Hello</h2>`), App[$.FILENAME], [[7, 2]]);
var root_1 = $.add_locations($.from_html(`<p></p>`), App[$.FILENAME], [[9, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let name = "world";
	var $$exports = { ...$.legacy_api() };
	{
		const title = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			var h2 = root();
			$.append($$anchor, h2);
		});
		$.add_svelte_meta(() => Card($$anchor, {
			title,
			children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
				var p = root_1();
				p.textContent = "Content world";
				$.append($$anchor, p);
			}),
			$$slots: {
				title: true,
				default: true
			}
		}), "component", App, 5, 0, { componentTag: "Card" });
	}
	return $.pop($$exports);
}
