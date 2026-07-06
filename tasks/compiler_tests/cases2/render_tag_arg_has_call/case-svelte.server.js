import * as $ from "svelte/internal/server";
function show($$renderer, data) {
	$$renderer.push(`<p>${$.escape(data)}</p>`);
}
export default function App($$renderer) {
	function getData() {
		return [
			1,
			2,
			3
		];
	}
	show($$renderer, getData());
}
