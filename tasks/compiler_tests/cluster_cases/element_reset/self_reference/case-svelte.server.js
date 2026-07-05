import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let file = $$props["file"];
		$$renderer.push(`<article${$.attr_class(`file ${$.stringify(file.type)}`)}><span class="name">${$.escape(file.name)}</span> `);
		if (file.type === "folder") {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<ul><!--[-->`);
			const each_array = $.ensure_array_like(file.children);
			for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
				let child = each_array[$$index];
				$$renderer.push(`<li>`);
				App($$renderer, { file: child });
				$$renderer.push(`<!----></li>`);
			}
			$$renderer.push(`<!--]--></ul>`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]--></article>`);
		$.bind_props($$props, { file });
	});
}
