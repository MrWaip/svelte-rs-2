import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { a: x = 5 } = $$props;
	$$renderer.push(`<button>${$.escape(x)}</button>`);
}
