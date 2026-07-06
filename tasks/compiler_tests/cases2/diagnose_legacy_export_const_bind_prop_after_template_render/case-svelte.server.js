import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const api = () => 1;
	$$renderer.push(`<div>hi</div>`);
	$.bind_props($$props, { api });
}
