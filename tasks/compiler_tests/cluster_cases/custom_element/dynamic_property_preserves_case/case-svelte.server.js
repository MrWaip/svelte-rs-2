import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let obj = $$props["obj"];
	$$renderer.push(`<my-element${$.attr("camelcase", obj)}></my-element>`);
	$.bind_props($$props, { obj });
}
