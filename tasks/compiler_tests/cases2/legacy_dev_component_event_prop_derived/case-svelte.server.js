import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	function handleClick() {}
	function getHandler() {
		return handleClick;
	}
	Child($$renderer, { onclick: getHandler() });
}
