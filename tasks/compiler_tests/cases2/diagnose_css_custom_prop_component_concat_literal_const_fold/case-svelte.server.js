import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const W = 120;
	$.css_props($$renderer, true, { "--cellWidth": "120px" }, () => {
		Child($$renderer, {});
	});
}
