import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let value = "red";
	const getSpread = () => ({ class: value });
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attributes({ ...getSpread() })}`);
	});
	$$renderer.push(` <button>toggle</button>`);
}
