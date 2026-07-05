import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { initial = "a" } = $$props;
	let selected = null;
	let dyn_val = initial;
	function rotate() {
		dyn_val = dyn_val + "!";
	}
	$$renderer.push(`<input type="radio"${$.attr("checked", selected === `item-${dyn_val}`, true)}${$.attr("value", `item-${dyn_val}`)}/> <button>rotate</button>`);
}
