import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let closeModal = $$props["closeModal"];
	$.bind_props($$props, { closeModal });
}
