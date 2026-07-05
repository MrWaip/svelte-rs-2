import * as $ from "svelte/internal/server";
import Widget from "./Widget.svelte";
export default function App($$renderer) {
	Widget($$renderer, { $$slots: { footer: ($$renderer) => {
		{
			$$renderer.push(`<p>First</p> <p>Second</p>`);
		}
	} } });
}
