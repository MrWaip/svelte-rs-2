App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
const $$css = {
	hash: "svelte-kpvy84",
	code: "\n	/* a comment */\n	.box.svelte-kpvy84 {\n		color: red;\n	}\n\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHVua25vd24pIiwic291cmNlcyI6WyIodW5rbm93bikiXSwic291cmNlc0NvbnRlbnQiOlsiPHN2ZWx0ZTpvcHRpb25zIGNzcz1cImluamVjdGVkXCIgLz5cblxuPHN0eWxlPlxuXHQvKiBhIGNvbW1lbnQgKi9cblx0LmJveCB7XG5cdFx0Y29sb3I6IHJlZDtcblx0fVxuPC9zdHlsZT5cblxuPGRpdiBjbGFzcz1cImJveFwiPmJveDwvZGl2PlxuIl0sIm5hbWVzIjpbXSwibWFwcGluZ3MiOiI7QUFHQTtBQUNBLENBQUMsa0JBQUksQ0FBQztBQUNOLEVBQUUsVUFBVTtBQUNaOyJ9 */"
};
function App($$renderer, $$props) {
	$$renderer.global.css.add($$css);
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div class="box svelte-kpvy84">`);
		$.push_element($$renderer, "div", 10, 0);
		$$renderer.push(`box</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
