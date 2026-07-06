import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { a = 10, b = 20 } = $$props;
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
}
