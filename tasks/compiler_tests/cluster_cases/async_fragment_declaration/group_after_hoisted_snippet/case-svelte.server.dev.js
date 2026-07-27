import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let name = "world";
		$$renderer.push(`<!--[-->`);
		{
			const sync = "sync";
			let number;
			let after_async;
			let length;
			let first;
			$.prevent_snippet_stringification(greet);
			function greet($$renderer) {
				$.validate_snippet_args($$renderer);
				let greeting;
				var promises_1 = $$renderer.run([async () => greeting = (await $.save(`Hello, ${name}!`))()]);
				$$renderer.push(`<h1>`);
				$.push_element($$renderer, "h1", 13, 2);
				$$renderer.async([promises_1[0]], ($$renderer) => $$renderer.push(() => $.escape(greeting)));
				$$renderer.push(`</h1>`);
				$.pop_element();
				$$renderer.push(` `);
				$$renderer.async([promises[0]], ($$renderer) => $$renderer.push(() => $.escape(number)));
				$$renderer.push(` `);
				$$renderer.async_block([
					promises[0],
					promises[1],
					promises_1[0]
				], ($$renderer) => {
					if (number > 4 && after_async && greeting) {
						$$renderer.push("<!--[0-->");
						let length;
						var promises_2 = $$renderer.run([() => promises[0], async () => length = (await $.save(number))()]);
						$$renderer.push(`<!--[-->`);
						$$renderer.async_block([promises_2[1]], ($$renderer) => {
							const each_array = $.ensure_array_like({ length });
							for (let index = 0, $$length = each_array.length; index < $$length; index++) {
								let i;
								var promises_3 = $$renderer.run([async () => i = (await $.save(index))()]);
								$$renderer.push(`<!---->`);
								$$renderer.async([promises_3[0]], ($$renderer) => $$renderer.push(() => $.escape(i)));
							}
						});
						$$renderer.push(`<!--]-->`);
					} else {
						$$renderer.push("<!--[-1-->");
					}
				});
				$$renderer.push(`<!--]-->`);
			}
			var promises = $$renderer.run([
				async () => number = (await $.save(Promise.resolve(5)))(),
				() => after_async = number + 1,
				async () => ({length, 0: first} = (await $.save("01234"))())
			]);
			greet($$renderer);
			$$renderer.push(`<!----> `);
			$$renderer.async([promises[0]], ($$renderer) => $$renderer.push(() => $.escape(number)));
			$$renderer.push(` sync `);
			$$renderer.async([promises[1]], ($$renderer) => $$renderer.push(() => $.escape(after_async)));
			$$renderer.push(` `);
			$$renderer.async([promises[2]], ($$renderer) => $$renderer.push(() => $.escape(length)));
			$$renderer.push(` `);
			$$renderer.async([promises[2]], ($$renderer) => $$renderer.push(() => $.escape(first)));
			$$renderer.push(` `);
			if (sync) {
				$$renderer.push("<!--[0-->");
				let double;
				var promises_4 = $$renderer.run([() => promises[0], () => double = number * 2]);
				$$renderer.async([promises_4[1]], ($$renderer) => $$renderer.push(() => $.escape(double)));
			} else {
				$$renderer.push("<!--[-1-->");
			}
			$$renderer.push(`<!--]-->`);
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
