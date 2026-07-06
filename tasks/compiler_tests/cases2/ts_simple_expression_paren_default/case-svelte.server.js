import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { x = 5 } = $$props;
	$$renderer.push(`<p>${$.escape(x)}</p>`);
}
