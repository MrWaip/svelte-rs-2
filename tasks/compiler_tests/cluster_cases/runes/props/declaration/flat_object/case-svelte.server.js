import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { a, b } = $$props;
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
}
