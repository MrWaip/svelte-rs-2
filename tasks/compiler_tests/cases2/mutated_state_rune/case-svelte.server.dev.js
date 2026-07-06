App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let title = 10;
		let flag = void 0;
		let flag2 = void 0;
		let value = "text";
		onMount(() => {
			title = 20;
			window.id = title;
			flag2 = title;
			map(title);
		});
		function map(value, off = title) {
			return value;
		}
		value += 1234;
		value -= 4e3;
		value *= 2;
		value &&= fallback;
		value = "";
		const obj = {
			title,
			title
		};
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 27, 0);
		$$renderer.push(`${$.escape(title)}</div>`);
		$.pop_element();
		$$renderer.push(` <div${$.attr("flag", flag)}>`);
		$.push_element($$renderer, "div", 29, 0);
		$$renderer.push(`${$.escape(flag2)}</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
