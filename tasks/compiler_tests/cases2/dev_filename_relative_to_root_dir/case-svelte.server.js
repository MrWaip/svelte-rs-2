import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { value } = $$props;
	$$renderer.push(`<div>${$.escape(value)}</div>`);
}
