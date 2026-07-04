App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
var root = $.add_locations($.from_html(`<p></p>`), App[$.FILENAME], [[8, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = 5;
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Widget($$anchor, {
		children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
			const doubled = $.tag($.derived(() => value * 2), "doubled");
			$.get(doubled);
			var p = root();
			p.textContent = $.get(doubled);
			$.append($$anchor, p);
		}),
		$$slots: { default: true }
	}), "component", App, 6, 0, { componentTag: "Widget" });
	return $.pop($$exports);
}
