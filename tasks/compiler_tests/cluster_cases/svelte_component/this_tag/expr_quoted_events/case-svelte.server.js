import * as $ from "svelte/internal/server";
import Foo from "./Foo.svelte";
import Bar from "./Bar.svelte";
export default function App($$renderer, $$props) {
	let x = $$props["x"];
	let selected = $$props["selected"];
	if (x ? Foo : Bar) {
		$$renderer.push("<!--[-->");
		(x ? Foo : Bar)($$renderer, {});
		$$renderer.push("<!--]-->");
	} else {
		$$renderer.push("<!--[!-->");
		$$renderer.push("<!--]-->");
	}
	$.bind_props($$props, {
		x,
		selected
	});
}
