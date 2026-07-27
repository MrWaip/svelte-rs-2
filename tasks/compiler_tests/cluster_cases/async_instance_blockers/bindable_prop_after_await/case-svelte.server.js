import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var value;
		var $$promises = $$renderer.run([() => Promise.resolve(), () => ({value = "test"} = $$props)]);
		$$renderer.push(`<button>update</button> `);
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(value)));
		$.bind_props($$props, { value });
	});
}
