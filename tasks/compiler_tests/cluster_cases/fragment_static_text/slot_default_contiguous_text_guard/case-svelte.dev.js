import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<x slot="s">1</x>`), App[$.FILENAME], [[1, 6]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => C($$anchor, {
		children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
			$.next();
			var text = $.text("OMG");
			$.append($$anchor, text);
		}),
		$$slots: {
			default: true,
			s: ($$anchor, $$slotProps) => {
				var x = root();
				$.append($$anchor, x);
			}
		}
	}), "component", App, 1, 0, { componentTag: "C" });
	return $.pop($$exports);
}
