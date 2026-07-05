import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let flag = true;
	const handler_1 = () => {};
	const handler_2 = () => {};
	let handler = $.derived(() => flag ? handler_1 : handler_2);
	$$renderer.push(`<button>x</button>`);
}
