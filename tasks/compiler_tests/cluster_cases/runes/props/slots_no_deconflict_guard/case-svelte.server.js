import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const { $$slots, $$events, ...props } = $$props;
	$$renderer.push(`<p>${$.escape(Object.keys(props))}</p>`);
}
