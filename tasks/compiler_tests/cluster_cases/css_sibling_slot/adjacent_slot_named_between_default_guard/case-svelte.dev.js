import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<div class="a svelte-1uoiiwh">a</div> <div class="c svelte-1uoiiwh">c</div>`, 1), App[$.FILENAME], [[6, 1], [8, 1]]);
var root_1 = $.add_locations($.from_html(`<div class="b" slot="wut">b</div>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, {
		children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
			var fragment_1 = root();
			$.next(2);
			$.append($$anchor, fragment_1);
		}),
		$$slots: {
			default: true,
			wut: ($$anchor, $$slotProps) => {
				var div = root_1();
				$.append($$anchor, div);
			}
		}
	}), "component", App, 5, 0, { componentTag: "Child" });
	return $.pop($$exports);
}
