import * as $ from "svelte/internal/server";
import Foo from "./Foo.svelte";
export default function App($$renderer, $$props) {
	let onclick = $.fallback($$props["onclick"], undefined);
	Foo($$renderer, { onclick });
	$.bind_props($$props, { onclick });
}
