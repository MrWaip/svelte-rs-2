import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let handler_a;
	let flag = true;
	const handler_1 = () => {};
	const handler_2 = () => {};
	$: handler_a = flag ? handler_1 : handler_2;
	$$renderer.push(`<button>x</button>`);
}
