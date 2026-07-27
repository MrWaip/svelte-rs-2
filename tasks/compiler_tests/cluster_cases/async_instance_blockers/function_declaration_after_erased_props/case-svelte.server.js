import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { p } = $$props;
	function f() {
		return 1;
	}
	var a, b;
	var $$promises = $$renderer.run([async () => a = await p, () => b = 2]);
	$$renderer.push(`<!---->`);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(a)));
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(b)));
	$$renderer.push(`${$.escape(f())}`);
}
