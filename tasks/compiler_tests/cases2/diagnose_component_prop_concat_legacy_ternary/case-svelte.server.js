import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
export default function App($$renderer, $$props) {
	let toggle = $.fallback($$props["toggle"], false);
	Comp($$renderer, { description: `prefix ${toggle ? "A" : "B"}` });
	$.bind_props($$props, { toggle });
}
