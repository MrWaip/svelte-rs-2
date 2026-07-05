import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { "a-b": ab } = $$props;
	$$renderer.push(`<button>${$.escape(ab)}</button>`);
}
