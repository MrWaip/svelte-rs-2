import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import A from "./A.svelte";
var root = $.add_locations($.from_html(`<div>x</div>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => A($$anchor, {
		children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
			var div = root();
			$.append($$anchor, div);
		}),
		$$slots: { default: true }
	}), "component", App, 5, 0, { componentTag: "A" });
	return $.pop($$exports);
}
