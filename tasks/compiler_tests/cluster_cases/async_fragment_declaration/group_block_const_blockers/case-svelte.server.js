import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<!--[-->`);
	{
		let number;
		function greet($$renderer) {
			let greeting;
			var promises_1 = $$renderer.run([async () => greeting = (await $.save("hi"))()]);
			$$renderer.async_block([promises[0], promises_1[0]], ($$renderer) => {
				if (number > 4 && greeting) {
					$$renderer.push("<!--[0-->");
					$$renderer.push(`<p>yes</p>`);
				} else {
					$$renderer.push("<!--[-1-->");
				}
			});
			$$renderer.push(`<!--]-->`);
		}
		var promises = $$renderer.run([async () => number = (await $.save(Promise.resolve(5)))()]);
		greet($$renderer);
	}
	$$renderer.push(`<!--]-->`);
}
