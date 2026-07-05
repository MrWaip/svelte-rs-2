import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { $$slots, $$events, ...props } = $$props;
	$$renderer.push(`<button${$.attributes({
		...props,
		is: "x-button"
	}, void 0, void 0, void 0, 2)}>x</button>`);
}
