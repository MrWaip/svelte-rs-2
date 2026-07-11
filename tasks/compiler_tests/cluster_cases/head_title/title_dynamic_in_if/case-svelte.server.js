import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let condition = $$props["condition"];
	let name = $$props["name"];
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		if (condition) {
			$$renderer.push("<!--[0-->");
			$$renderer.title(($$renderer) => {
				$$renderer.push(`<title>Hi ${$.escape(name)}</title>`);
			});
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
	});
	$.bind_props($$props, {
		condition,
		name
	});
}
