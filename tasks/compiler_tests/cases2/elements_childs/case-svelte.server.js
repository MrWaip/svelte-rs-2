import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div>text only</div> <div>${$.escape(interpolation)}</div> <div>concatenated + ${$.escape(interpolation)} + concatenated</div> <div><div>more nested</div> <div>more nested</div> <div>more nested</div></div> <div>`);
	if (1 !== 1) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<div></div>`);
	} else if (2 === 2) {
		$$renderer.push("<!--[1-->");
		$$renderer.push(`<div></div>`);
	} else {
		$$renderer.push("<!--[-1-->");
		$$renderer.push(`<div></div>`);
	}
	$$renderer.push(`<!--]--></div> <div></div>`);
}
