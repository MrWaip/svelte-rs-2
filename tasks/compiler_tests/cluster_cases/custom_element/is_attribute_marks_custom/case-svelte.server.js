import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let obj = $$props["obj"];
	$$renderer.push(`<button is="my-button"${$.attr("foo", obj)}></button>`);
	$.bind_props($$props, { obj });
}
