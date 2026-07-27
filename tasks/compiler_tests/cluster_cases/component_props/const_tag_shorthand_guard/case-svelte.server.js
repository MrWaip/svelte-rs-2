import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	let live = 0;
	let plain = 7;
	function bump() {
		live++;
	}
	function row($$renderer) {
		const kLit = "x";
		const kLive = live + 1;
		const kCall = Math.random();
		Child($$renderer, {
			kLive,
			kCall,
			plain,
			eLit: kLit,
			eLive: kLive
		});
	}
	row($$renderer);
	$$renderer.push(`<!----> <button>b</button>`);
}
