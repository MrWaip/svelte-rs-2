import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	function getX() {
		return 1;
	}
	Child($$renderer, { random: getX() });
}
