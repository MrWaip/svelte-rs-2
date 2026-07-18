App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
const $$css = {
	hash: "svelte-472ibj",
	code: "\n	/* :global {*/\n		.box {\n			color: red;\n		}\n	/*}*/\n\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHVua25vd24pIiwic291cmNlcyI6WyIodW5rbm93bikiXSwic291cmNlc0NvbnRlbnQiOlsiPHN2ZWx0ZTpvcHRpb25zIGNzcz1cImluamVjdGVkXCIgLz5cblxuPHN0eWxlPlxuXHQ6Z2xvYmFsIHtcblx0XHQuYm94IHtcblx0XHRcdGNvbG9yOiByZWQ7XG5cdFx0fVxuXHR9XG48L3N0eWxlPlxuXG48ZGl2IGNsYXNzPVwiYm94XCI+Ym94PC9kaXY+XG4iXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IjtBQUdBLElBQUMsUUFBUTtBQUNULEVBQUUsSUFBSSxDQUFDO0FBQ1AsR0FBRyxVQUFVO0FBQ2I7QUFDQSxHQUFDOyJ9 */"
};
function App($$renderer, $$props) {
	$$renderer.global.css.add($$css);
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div class="box">`);
		$.push_element($$renderer, "div", 11, 0);
		$$renderer.push(`box</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
