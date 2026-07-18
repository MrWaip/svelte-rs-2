App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
const $$css = {
	hash: "svelte-limxtm",
	code: "\n	.a.svelte-limxtm,\n	.b.svelte-limxtm {\n		color: red;\n	}\n\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHVua25vd24pIiwic291cmNlcyI6WyIodW5rbm93bikiXSwic291cmNlc0NvbnRlbnQiOlsiPHN2ZWx0ZTpvcHRpb25zIGNzcz1cImluamVjdGVkXCIgLz5cblxuPHN0eWxlPlxuXHQuYSxcblx0LmIge1xuXHRcdGNvbG9yOiByZWQ7XG5cdH1cbjwvc3R5bGU+XG5cbjxkaXYgY2xhc3M9XCJhXCI+YTwvZGl2PlxuPGRpdiBjbGFzcz1cImJcIj5iPC9kaXY+XG4iXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IjtBQUdBLENBQUMsZ0JBQUU7QUFDSCxDQUFDLGdCQUFFLENBQUM7QUFDSixFQUFFLFVBQVU7QUFDWjsifQ== */"
};
function App($$renderer, $$props) {
	$$renderer.global.css.add($$css);
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div class="a svelte-limxtm">`);
		$.push_element($$renderer, "div", 10, 0);
		$$renderer.push(`a</div>`);
		$.pop_element();
		$$renderer.push(` <div class="b svelte-limxtm">`);
		$.push_element($$renderer, "div", 11, 0);
		$$renderer.push(`b</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
