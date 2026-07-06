import * as $ from "svelte/internal/server";
function card($$renderer, heading) {
	$$renderer.push(`<div><h3>${$.escape(heading)}</h3> `);
	badge($$renderer, "new");
	$$renderer.push(`<!----></div>`);
}
export default function App($$renderer) {
	function action(node, arg) {
		return { destroy() {} };
	}
}
