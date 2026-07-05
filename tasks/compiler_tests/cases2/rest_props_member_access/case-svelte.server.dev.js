App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { id, $$slots, $$events, ...props } = $$props;
		const label = $.derived(() => props.label + "!");
		const style = $.derived(() => props.style);
		const color = $.derived(() => props.style.color);
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 7, 0);
		$$renderer.push(`${$.escape(label())}</p>`);
		$.pop_element();
		$$renderer.push(` <span>`);
		$.push_element($$renderer, "span", 8, 0);
		$$renderer.push(`${$.escape(props.title)}</span>`);
		$.pop_element();
		$$renderer.push(` <div>`);
		$.push_element($$renderer, "div", 9, 0);
		$$renderer.push(`${$.escape(props.nested.deep.value)}</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
