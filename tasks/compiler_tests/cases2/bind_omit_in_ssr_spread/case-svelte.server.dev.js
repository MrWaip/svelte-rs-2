App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let rest = {};
		let w = 0;
		let rect = void 0;
		let time = 0;
		let ind = false;
		let files = void 0;
		let el;
		let val = "";
		let checked = false;
		let open = false;
		$$renderer.push(`<div${$.attributes({ ...rest })}>`);
		$.push_element($$renderer, "div", 14, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
		$$renderer.push(` <video${$.attributes({ ...rest })}>`);
		$.push_element($$renderer, "video", 15, 0);
		$$renderer.push(`</video>`);
		$.pop_element();
		$$renderer.push(` <input${$.attributes({
			type: "checkbox",
			...rest
		}, void 0, void 0, void 0, 4)}/>`);
		$.push_element($$renderer, "input", 16, 0);
		$.pop_element();
		$$renderer.push(` <input${$.attributes({
			type: "file",
			...rest
		}, void 0, void 0, void 0, 4)}/>`);
		$.push_element($$renderer, "input", 17, 0);
		$.pop_element();
		$$renderer.push(` <input${$.attributes({
			value: val,
			...rest
		}, void 0, void 0, void 0, 4)}/>`);
		$.push_element($$renderer, "input", 19, 0);
		$.pop_element();
		$$renderer.push(` <input${$.attributes({
			type: "checkbox",
			checked,
			...rest
		}, void 0, void 0, void 0, 4)}/>`);
		$.push_element($$renderer, "input", 20, 0);
		$.pop_element();
		$$renderer.push(` <details${$.attributes({
			open,
			...rest
		})}>`);
		$.push_element($$renderer, "details", 21, 0);
		$$renderer.push(`</details>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
