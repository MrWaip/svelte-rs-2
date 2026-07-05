import * as $ from "svelte/internal/server";
import Widget from "./Widget.svelte";
export default function App($$renderer, $$props) {
	let { value = "x" } = $$props;
	Widget($$renderer, { $$slots: { footer: ($$renderer) => {
		$$renderer.push(`<div slot="footer">Footer: ${$.escape(value)}</div>`);
	} } });
}
