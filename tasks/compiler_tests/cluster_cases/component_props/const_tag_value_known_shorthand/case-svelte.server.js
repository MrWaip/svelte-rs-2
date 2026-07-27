import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	let plain = 7;
	function row($$renderer) {
		const kLit = "x";
		const kArith = plain + 1;
		Child($$renderer, {
			kLit,
			kArith
		});
	}
	row($$renderer);
}
