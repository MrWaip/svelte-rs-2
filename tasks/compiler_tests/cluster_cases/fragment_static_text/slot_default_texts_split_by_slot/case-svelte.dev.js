import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`foobar`, 1), App[$.FILENAME], []);
var root_1 = $.add_locations($.from_html(`<x slot="s">y</x>`), App[$.FILENAME], [[1, 6]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => C($$anchor, {
		children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
			$.next();
			var fragment_1 = root();
			$.append($$anchor, fragment_1);
		}),
		$$slots: {
			default: true,
			s: ($$anchor, $$slotProps) => {
				var x = root_1();
				$.append($$anchor, x);
			}
		}
	}), "component", App, 1, 0, { componentTag: "C" });
	return $.pop($$exports);
}
