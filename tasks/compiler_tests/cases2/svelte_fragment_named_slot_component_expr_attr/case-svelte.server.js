import * as $ from "svelte/internal/server";
import Outer from "./Outer.svelte";
import Inner from "./Inner.svelte";
export default function App($$renderer, $$props) {
	let value = $$props["value"];
	Outer($$renderer, { $$slots: { content: ($$renderer) => {
		{
			Inner($$renderer, { prop: value });
		}
	} } });
	$.bind_props($$props, { value });
}
