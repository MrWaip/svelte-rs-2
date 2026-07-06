import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let obj = $$props["obj"];
		if (obj) {
			$$renderer.push("<!--[0-->");
			const name = obj.name;
			const len = name.length;
			$$renderer.push(`<span>${$.escape(name)}: ${$.escape(len)}</span>`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { obj });
	});
}
