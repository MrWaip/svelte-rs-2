import * as $ from "svelte/internal/server";
import Foo from "./Foo.svelte";
export default function App($$renderer) {
	if (Foo) {
		$$renderer.push("<!--[0-->");
		const Component = Foo;
		Component($$renderer, {});
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
