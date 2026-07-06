import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
export default function App($$renderer) {
	let i = 0;
	let index = 0;
	function bump() {
		i++;
	}
	Comp($$renderer, { active: i === index });
	$$renderer.push(`<!----> <button>bump</button>`);
}
