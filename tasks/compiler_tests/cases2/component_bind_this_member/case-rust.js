import * as $ from "svelte/internal/client";
import Component from "./Component.svelte";
export default function App($$anchor) {
	let obj = { ref: null };
	$.bind_this(Component($$anchor, {}), ($$value) => obj.ref = $$value, () => obj?.ref);
}
