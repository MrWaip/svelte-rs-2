import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function test() {}
	$$renderer.select({ value: test() }, ($$renderer) => {});
}
