import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const { i, j } = {
		i: 1,
		j: 2
	};
	$$renderer.push(`<p>${$.escape(i)}${$.escape(j)}</p>`);
	$.bind_props($$props, { i });
}
