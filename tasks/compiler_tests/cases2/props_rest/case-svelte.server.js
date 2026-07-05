import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { x, $$slots, $$events, ...rest } = $$props;
	$$renderer.push(`<p>${$.escape(x)}</p>`);
}
