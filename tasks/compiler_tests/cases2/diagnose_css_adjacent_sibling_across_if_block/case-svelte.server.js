import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { x, y } = $$props;
	$$renderer.push(`<div class="a svelte-13830z5"></div> `);
	if (x) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<div class="b svelte-13830z5"></div>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--> `);
	if (y) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<div class="c svelte-13830z5"></div>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
