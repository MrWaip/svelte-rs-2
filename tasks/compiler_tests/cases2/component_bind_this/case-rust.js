import * as $ from "svelte/internal/client";
import Component from "./Component.svelte";
export default function App($$anchor) {
	let ref = $.state(void 0);
	$.bind_this(Component($$anchor, {}), ($$value) => $.set(ref, $$value, true), () => $.get(ref));
}
