App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
var root = $.add_locations($.from_html(`<div slot="a"></div>`), App[$.FILENAME], [[6, 4]]);
var root_1 = $.add_locations($.from_html(`<div slot="b"></div>`), App[$.FILENAME], [[7, 4]]);
var root_2 = $.add_locations($.from_html(`<div slot="c">x</div>`), App[$.FILENAME], [[8, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Inner($$anchor, { $$slots: {
		a: ($$anchor, $$slotProps) => {
			var div = root();
			$.append($$anchor, div);
		},
		b: ($$anchor, $$slotProps) => {
			var div_1 = root_1();
			$.append($$anchor, div_1);
		},
		c: ($$anchor, $$slotProps) => {
			var div_2 = root_2();
			$.append($$anchor, div_2);
		}
	} }), "component", App, 5, 0, { componentTag: "Inner" });
	return $.pop($$exports);
}
