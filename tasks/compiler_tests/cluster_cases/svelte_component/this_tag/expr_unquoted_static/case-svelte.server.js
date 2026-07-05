import * as $ from "svelte/internal/server";
import Foo from "./Foo.svelte";
export default function App($$renderer) {
	if (Foo) {
		$$renderer.push("<!--[-->");
		Foo($$renderer, {});
		$$renderer.push("<!--]-->");
	} else {
		$$renderer.push("<!--[!-->");
		$$renderer.push("<!--]-->");
	}
}
