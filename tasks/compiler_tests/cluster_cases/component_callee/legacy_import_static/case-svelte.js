import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Foo from "./Foo.svelte";
export default function App($$anchor) {
	Foo($$anchor, {});
}
