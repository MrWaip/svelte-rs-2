import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { items = [], config = getDefault(), label = "hello" } = $$props;
	$$renderer.push(`<p>${$.escape(label)}</p>`);
}
