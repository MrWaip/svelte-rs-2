import * as $ from "svelte/internal/server";
import Row from "./Row.svelte";
export default function App($$renderer, $$props) {
	let { n } = $$props;
	function compute() {
		return n + 1;
	}
	{
		function cell($$renderer) {
			$$renderer.push(`<p>cell</p>`);
		}
		Row($$renderer, {
			cell,
			children: ($$renderer) => {
				const value = compute();
				$$renderer.push(`<div>${$.escape(value)}</div>`);
			},
			$$slots: {
				cell: true,
				default: true
			}
		});
	}
}
