import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let group = [];
	$$renderer.push(`<input type="radio"${$.attr("checked", group === "a", true)} value="a"/> <input type="radio"${$.attr("checked", group === "b", true)} value="b"/> <input type="radio"${$.attr("checked", group === "c", true)} value="c"/>`);
}
