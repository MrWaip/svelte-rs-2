import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	let ref = $.mutable_source();
	$.bind_this(Child($$anchor, { $$legacy: true }), ($$value) => $.set(ref, $$value), () => $.get(ref));
}
