import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Foo from "./Foo.svelte";
export default function App($$anchor, $$props) {
	let onclick = $.prop($$props, "onclick", 8, undefined);
	Foo($$anchor, { get onclick() {
		return onclick();
	} });
}
