import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { a: x, b: y } = $$props;
	$$renderer.push(`<button>${$.escape(x)}${$.escape(y)}</button>`);
}
