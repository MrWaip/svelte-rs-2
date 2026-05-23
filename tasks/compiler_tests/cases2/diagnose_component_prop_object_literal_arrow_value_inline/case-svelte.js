import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
import { invalidate } from "./lib";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	Child($$anchor, { props: { onStart: () => invalidate(true) } });
	$.pop();
}
