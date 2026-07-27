import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	var d, k, m, y;
	var $$promises = $$renderer.run([
		async () => d = await fetch("/a"),
		() => k = 5,
		async () => m = await fetch("/b"),
		() => y = 1
	]);
	$$renderer.push(`<!---->`);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(d)));
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(k)));
	$$renderer.async([$$promises[2]], ($$renderer) => $$renderer.push(() => $.escape(m)));
	$$renderer.async([$$promises[3]], ($$renderer) => $$renderer.push(() => $.escape(y)));
	$.bind_props($$props, { k });
}
