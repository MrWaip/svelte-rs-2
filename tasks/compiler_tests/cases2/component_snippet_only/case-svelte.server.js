import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { data } = $$props;
	{
		function row($$renderer, item) {
			$$renderer.push(`<span>${$.escape(item)}</span>`);
		}
		Table($$renderer, {
			items: data,
			row,
			$$slots: { row: true }
		});
	}
}
