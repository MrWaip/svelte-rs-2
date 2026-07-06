import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
import Outer from "./Outer.svelte";
export default function App($$renderer, $$props) {
	let value = $$props["value"];
	Outer($$renderer, { $$slots: { footer: ($$renderer) => {
		Inner($$renderer, {
			slot: "footer",
			x: value
		});
	} } });
	$.bind_props($$props, { value });
}
