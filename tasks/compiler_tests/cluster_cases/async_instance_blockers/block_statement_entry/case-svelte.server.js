import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var data, y;
	var $$promises = $$renderer.run([async () => data = await fetch("/a"), () => {
		{
			console.log(1);
			console.log(2);
		}
		y = 1;
	}]);
	$$renderer.push(`<!---->`);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(data)));
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(y)));
}
