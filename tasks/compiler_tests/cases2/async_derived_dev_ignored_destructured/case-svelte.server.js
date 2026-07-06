import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var name, age;
	var $$promises = $$renderer.run([async () => {
		var $$d = await $.async_derived(() => fetch("/api"));
		name = $.derived(() => $$d().name);
		age = $.derived(() => $$d().age);
	}]);
	$$renderer.push(`<p>`);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(name())));
	$$renderer.push(` `);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(age())));
	$$renderer.push(`</p>`);
}
