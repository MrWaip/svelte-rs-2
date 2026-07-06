import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const { $$slots, $$events, ...props } = $$props;
	const a = $.derived(() => props.a), b = $.derived(() => props.b);
	$$renderer.push(`<p>${$.escape(a())},${$.escape(b())}</p>`);
}
