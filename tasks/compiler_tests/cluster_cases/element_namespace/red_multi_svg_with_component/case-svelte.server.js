import * as $ from "svelte/internal/server";
import Foo from "./Foo.svelte";
export default function App($$renderer) {
	$$renderer.push(`<svg></svg><svg></svg>`);
	Foo($$renderer, {});
	$$renderer.push(`<!---->`);
}
