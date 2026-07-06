import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let url = "/api";
		var data, meta;
		var $$promises = $$renderer.run([async () => {
			var $$d = await $.async_derived(() => fetch(url).then((r) => r.json()));
			data = $.derived(() => $$d().data);
			meta = $.derived(() => $$d().meta);
		}]);
		$$renderer.push(`<p>`);
		$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(data())));
		$$renderer.push(`-`);
		$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(meta())));
		$$renderer.push(`</p>`);
	});
}
