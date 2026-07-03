import * as $ from "svelte/internal/client";
import Row from "./Row.svelte";
var root = $.from_html(`<p>cell</p>`);
var root_1 = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	function compute() {
		return $$props.n + 1;
	}
	{
		const cell = ($$anchor) => {
			var p = root();
			$.append($$anchor, p);
		};
		Row($$anchor, {
			cell,
			children: ($$anchor, $$slotProps) => {
				const value = $.derived(compute);
				var div = root_1();
				var text = $.child(div, true);
				$.reset(div);
				$.template_effect(() => $.set_text(text, $.get(value)));
				$.append($$anchor, div);
			},
			$$slots: {
				cell: true,
				default: true
			}
		});
	}
}
