import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { a, b } = $$props;
	$$renderer.push(`<table><tbody><tr><td>`);
	if (a) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`${$.escape(a)} <br/>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--> `);
	if (b) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`${$.escape(b)} <br/>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--></td></tr></tbody></table>`);
}
