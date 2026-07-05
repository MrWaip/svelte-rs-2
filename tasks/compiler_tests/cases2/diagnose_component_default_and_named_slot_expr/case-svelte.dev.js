App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
var root = $.add_locations($.from_html(`<div slot="footer"></div>`), App[$.FILENAME], [[8, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let counter = 0;
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Inner($$anchor, {
		children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
			$.next();
			var text = $.text("default text");
			$.append($$anchor, text);
		}),
		$$slots: {
			default: true,
			footer: ($$anchor, $$slotProps) => {
				var div = root();
				div.textContent = "Footer: 0";
				$.append($$anchor, div);
			}
		}
	}), "component", App, 6, 0, { componentTag: "Inner" });
	return $.pop($$exports);
}
