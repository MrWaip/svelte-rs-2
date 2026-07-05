import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { a } = $$props;
	$$renderer.push(`<button>${$.escape(a)}</button>`);
}
