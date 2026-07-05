import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
export default function App($$renderer, $$props) {
	let value = $$props["value"];
	Comp($$renderer, { id: value });
	$.bind_props($$props, { value });
}
