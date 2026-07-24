import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { stuff } = $$props;
	function bump() {
		stuff = 5;
	}
	$.bind_props($$props, { stuff });
}
