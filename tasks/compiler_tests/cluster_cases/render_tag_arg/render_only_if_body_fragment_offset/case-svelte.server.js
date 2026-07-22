import * as $ from "svelte/internal/server";
function co($$renderer) {
	$$renderer.push(`<b>C</b>`);
}
export default function App($$renderer) {
	let show = true;
	if (show) {
		$$renderer.push("<!--[0-->");
		co($$renderer);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--> <select>`);
	$$renderer.option({}, ($$renderer) => {
		$$renderer.push(`<span>M</span>`);
	}, void 0, void 0, void 0, void 0, true);
	$$renderer.push(`</select>`);
}
