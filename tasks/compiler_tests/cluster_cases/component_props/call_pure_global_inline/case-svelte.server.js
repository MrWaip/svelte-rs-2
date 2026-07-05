import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Child($$renderer, { random: Math.random().toFixed(2) });
	});
}
