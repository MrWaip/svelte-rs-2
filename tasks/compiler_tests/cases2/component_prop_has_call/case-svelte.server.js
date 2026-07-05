import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
export default function App($$renderer) {
	function getData() {
		return [
			1,
			2,
			3
		];
	}
	Comp($$renderer, { data: getData() });
}
