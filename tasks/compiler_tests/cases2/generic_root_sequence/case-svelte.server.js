import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<!---->some text <div></div> ${$.escape(some_variable)} <input/> text + ${$.escape(name)} <div></div> `);
	if (true) {
		$$renderer.push("<!--[0-->");
	} else if (false) {
		$$renderer.push("<!--[1-->");
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
