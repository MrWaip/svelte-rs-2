import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = "";
	$$renderer.push(`<set-property-before-mounted${$.attr("property", value)}></set-property-before-mounted> <set-property-before-mounted${$.attr("property", value)}></set-property-before-mounted>`);
}
