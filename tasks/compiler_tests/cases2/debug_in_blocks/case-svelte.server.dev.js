App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let show = true;
		let x = 42;
		let items = [
			1,
			2,
			3
		];
		if (show) {
			$$renderer.push("<!--[0-->");
			console.log({ x });
			debugger;
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]--> <!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			console.log({ item });
			debugger;
		}
		$$renderer.push(`<!--]--> <div>`);
		console.log({ x });
		debugger;
		$.push_element($$renderer, "div", 15, 0);
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 17, 1);
		$$renderer.push(`Value: 42</p>`);
		$.pop_element();
		$$renderer.push(`</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
