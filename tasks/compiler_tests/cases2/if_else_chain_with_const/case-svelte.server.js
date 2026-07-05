import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	let name = "world";
	if (count > 100) {
		$$renderer.push("<!--[0-->");
		const label = name + "!";
		$$renderer.push(`<h1>world!</h1>`);
	} else if (count > 50) {
		$$renderer.push("<!--[1-->");
		$$renderer.push(`<h2>Medium: 0</h2>`);
	} else if (count > 10) {
		$$renderer.push("<!--[2-->");
		const small = count * 2;
		$$renderer.push(`<h3>Small doubled: 0</h3>`);
	} else {
		$$renderer.push("<!--[-1-->");
		$$renderer.push(`<p>Tiny: 0</p>`);
	}
	$$renderer.push(`<!--]-->`);
}
