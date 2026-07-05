import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let flag = $.fallback($$props["flag"], false);
	let Comp = $$props["Comp"];
	function onA() {}
	function onB() {}
	if (Comp) {
		$$renderer.push("<!--[-->");
		Comp($$renderer, {
			onA: flag ? onA : undefined,
			onB: flag ? onB : undefined
		});
		$$renderer.push("<!--]-->");
	} else {
		$$renderer.push("<!--[!-->");
		$$renderer.push("<!--]-->");
	}
	$.bind_props($$props, {
		flag,
		Comp
	});
}
