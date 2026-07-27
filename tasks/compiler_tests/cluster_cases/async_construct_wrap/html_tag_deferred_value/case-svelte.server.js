import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var html;
	var $$promises = $$renderer.run([async () => html = await Promise.resolve("<b>hi</b>")]);
	$$renderer.push(`<div>`);
	$$renderer.async_block([$$promises[0]], ($$renderer) => {
		$$renderer.push($.html(html));
	});
	$$renderer.push(`</div>`);
}
