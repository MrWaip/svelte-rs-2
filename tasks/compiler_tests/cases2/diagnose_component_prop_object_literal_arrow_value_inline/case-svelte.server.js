import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
import { invalidate } from "./lib";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Child($$renderer, { props: { onStart: () => invalidate(true) } });
	});
}
