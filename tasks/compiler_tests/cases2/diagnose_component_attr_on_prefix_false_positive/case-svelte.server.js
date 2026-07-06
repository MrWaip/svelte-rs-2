import * as $ from "svelte/internal/server";
import Foo from "./Foo.svelte";
export default function App($$renderer) {
	Foo($$renderer, { onlyOrder: true });
}
