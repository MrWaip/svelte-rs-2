import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Foo from "./Foo.svelte";
export default function App($$anchor, $$props) {
	let foo = $.prop($$props, "foo", 12);
	$.bind_this(Foo($$anchor, { $$legacy: true }), ($$value) => foo($$value), () => foo());
}
