import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$.await($$renderer, promise, () => {}, (result) => {
		$$renderer.push(`text ${$.escape(result.name)} <div>${$.escape(result.value)}</div>`);
	});
	$$renderer.push(`<!--]-->`);
}
