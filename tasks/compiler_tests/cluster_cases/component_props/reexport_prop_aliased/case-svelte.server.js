import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { stuff } = $$props;
	$.bind_props($$props, { cool: stuff });
}
