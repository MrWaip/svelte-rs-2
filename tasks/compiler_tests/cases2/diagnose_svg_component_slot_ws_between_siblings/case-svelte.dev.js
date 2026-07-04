App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_svg(`<g><path d="M1"></path></g><g><path d="M2"></path></g>`, 1), App[$.FILENAME], [[
	6,
	1,
	[[6, 4]]
], [
	7,
	1,
	[[7, 4]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, {
		children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
			var fragment_1 = root();
			$.next();
			$.append($$anchor, fragment_1);
		}),
		$$slots: { default: true }
	}), "component", App, 5, 0, { componentTag: "Child" });
	return $.pop($$exports);
}
