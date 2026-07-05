import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const greet = () => "hi";
	$.bind_props($$props, { greet });
}
