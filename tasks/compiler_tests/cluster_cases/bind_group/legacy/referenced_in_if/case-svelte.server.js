import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let test = $.fallback($$props["test"], () => [], true);
	let hidden = false;
	$$renderer.push(`<button>${$.escape(hidden ? "show" : "hide")} b</button> <label>a <input type="checkbox"${$.attr("checked", test.includes("a"), true)} value="a"/></label> `);
	if (!hidden) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<label>b <input type="checkbox"${$.attr("checked", test.includes("b"), true)} value="b"/></label>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--> <label>c${$.escape(hidden ? "show" : "hide")} b <input value="just here, so b is not the last input"/></label>`);
	$.bind_props($$props, { test });
}
