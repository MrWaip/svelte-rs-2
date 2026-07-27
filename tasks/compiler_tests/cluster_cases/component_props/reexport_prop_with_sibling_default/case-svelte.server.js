import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { count = 0, stuff } = $$props;
	$.bind_props($$props, {
		count,
		stuff
	});
}
