import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
import { tracker } from "./tracker";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Child($$renderer, { track: tracker.click.upgrade() });
	});
}
