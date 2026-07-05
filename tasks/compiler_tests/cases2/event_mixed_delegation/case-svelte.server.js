import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	function handleClick() {
		count++;
	}
	function getHandler() {
		return handleClick;
	}
	$$renderer.push(`<div>${$.escape(count)}</div>`);
}
