import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var message;
	var $$promises = $$renderer.run([() => Promise.resolve(), () => message = $.derived(() => "hello")]);
	$$renderer.push(`<p>`);
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(message())));
	$$renderer.push(`</p>`);
}
