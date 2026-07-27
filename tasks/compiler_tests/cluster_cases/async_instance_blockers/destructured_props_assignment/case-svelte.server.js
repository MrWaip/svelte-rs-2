import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	var loaded, name;
	var $$promises = $$renderer.run([async () => loaded = await Promise.resolve(1), () => ({name} = $$props)]);
	$$renderer.push(`<p>`);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(loaded)));
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(name)));
	$$renderer.push(`</p>`);
}
