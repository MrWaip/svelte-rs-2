import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { value = "a" } = $$props;
	$$renderer.push(`<input type="radio"${$.attr("checked", value === "a", true)} value="a"/>`);
}
