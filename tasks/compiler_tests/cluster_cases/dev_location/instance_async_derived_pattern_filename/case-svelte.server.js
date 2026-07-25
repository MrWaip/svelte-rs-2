import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { p } = $$props;
	var first, second;
	var $$promises = $$renderer.run([async () => {
		var $$d = await $.async_derived(() => p);
		first = $.derived(() => $$d().first);
		second = $.derived(() => $$d().second);
	}]);
	$$renderer.push(`<p>`);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(first())));
	$$renderer.push(` `);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(second())));
	$$renderer.push(`</p>`);
}
