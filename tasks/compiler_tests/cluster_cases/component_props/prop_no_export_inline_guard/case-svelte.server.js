import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { stuff } = $$props;
	$$renderer.push(`<p>${$.escape(stuff)}</p>`);
}
