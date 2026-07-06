import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	function handleEvent() {
		count++;
	}
	function action(node) {
		return { destroy() {} };
	}
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		$$renderer.title(($$renderer) => {
			$$renderer.push(`<title>App: ${$.escape(count)}</title>`);
		});
		$$renderer.push(`<meta name="viewport" content="width=device-width"/>`);
	});
	$$renderer.push(`<div><p>Count: ${$.escape(count)}</p></div>`);
}
