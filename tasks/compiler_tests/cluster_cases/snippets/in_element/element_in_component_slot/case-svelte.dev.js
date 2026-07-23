App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<b>hi</b>`), App[$.FILENAME], [[1, 32]]);
var root_1 = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[1, 11]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Component($$anchor, {
		children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
			var div = root_1();
			{
				const foo = $.wrap_snippet(App, function($$anchor) {
					$.validate_snippet_args(...arguments);
					var b = root();
					$.append($$anchor, b);
				});
			}
			$.append($$anchor, div);
		}),
		$$slots: { default: true }
	}), "component", App, 1, 0, { componentTag: "Component" });
	return $.pop($$exports);
}
