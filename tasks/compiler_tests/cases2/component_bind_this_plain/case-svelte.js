import * as $ from "svelte/internal/client";
import Component from "./Component.svelte";
export default function App($$anchor) {
	let ref;
	$.bind_this(Component($$anchor, {}), ($$value) => ref = $$value, () => ref);
}
