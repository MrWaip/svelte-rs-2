import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let foo = $$props["foo"];
	foo;
	$$renderer.push(`<div>hi</div>`);
	$.bind_props($$props, { foo });
}
