import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
var root_1 = $.from_html(`<div slot="a"></div>`);
var root_2 = $.from_html(`<div slot="b"></div>`);
var root_3 = $.from_html(`<div slot="c">x</div>`);
export default function App($$anchor) {
	Inner($$anchor, { $$slots: {
		a: ($$anchor, $$slotProps) => {
			var div = root_1();
			$.append($$anchor, div);
		},
		b: ($$anchor, $$slotProps) => {
			var div_1 = root_2();
			$.append($$anchor, div_1);
		},
		c: ($$anchor, $$slotProps) => {
			var div_2 = root_3();
			$.append($$anchor, div_2);
		}
	} });
}
