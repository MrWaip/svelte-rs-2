import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let count = 0;
	let { val = count } = $$props;
	$$renderer.push(`<p>${$.escape(val)}</p>`);
}
