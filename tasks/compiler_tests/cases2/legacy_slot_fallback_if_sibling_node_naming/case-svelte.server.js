import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let show = $$props["show"];
	let value = $$props["value"];
	$$renderer.push(`<li><!--[-->`);
	$.slot($$renderer, $$props, "item", {}, () => {
		if (show) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<span>${$.escape(value)}</span>`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]--> <div>tail</div>`);
	});
	$$renderer.push(`<!--]--></li>`);
	$.bind_props($$props, {
		show,
		value
	});
}
