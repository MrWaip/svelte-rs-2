import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor) {
	let comp = $.state(void 0);
	$.bind_this(Comp($$anchor, {}), (v) => $.set(comp, v, true), () => $.get(comp));
}
