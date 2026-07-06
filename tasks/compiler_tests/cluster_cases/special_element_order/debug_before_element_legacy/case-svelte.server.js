import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let count = $$props["count"];
	console.log({ count });
	debugger;
	$$renderer.push(`<button>+</button>`);
	$.bind_props($$props, { count });
}
