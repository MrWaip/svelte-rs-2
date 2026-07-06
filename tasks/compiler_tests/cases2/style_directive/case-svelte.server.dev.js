App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let color = "red";
		let fontSize = "16px";
		let bg = "blue";
		let columns = 3;
		const staticVal = "bold";
		$$renderer.push(`<div${$.attr_style("", {
			color,
			"--columns": columns,
			"font-size": fontSize,
			"background-color": bg,
			"font-weight": staticVal
		})}>`);
		$.push_element($$renderer, "div", 9, 0);
		$$renderer.push(`Styled</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
