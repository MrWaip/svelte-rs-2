import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let trigger = $$props["trigger"];
	let value;
	function read() {
		return value;
	}
	$: value = trigger;
	$$renderer.push(`<button></button>`);
	$.bind_props($$props, { trigger });
}
