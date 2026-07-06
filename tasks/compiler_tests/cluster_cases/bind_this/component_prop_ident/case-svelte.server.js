import * as $ from "svelte/internal/server";
import Foo from "./Foo.svelte";
export default function App($$renderer, $$props) {
	let foo = $$props["foo"];
	Foo($$renderer, {});
	$.bind_props($$props, { foo });
}
