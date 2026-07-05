import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span slot="action"></span>`), App[$.FILENAME], [[7, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let x = 0;
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Wrap($$anchor, { $$slots: {
		image: ($$anchor, $$slotProps) => {
			$.add_svelte_meta(() => Inner($$anchor, { slot: "image" }), "component", App, 6, 4, { componentTag: "Inner" });
		},
		action: ($$anchor, $$slotProps) => {
			var span = root();
			span.textContent = "0";
			$.append($$anchor, span);
		}
	} }), "component", App, 5, 0, { componentTag: "Wrap" });
	return $.pop($$exports);
}
