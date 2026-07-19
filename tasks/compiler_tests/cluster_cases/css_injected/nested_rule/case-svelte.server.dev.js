App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
const $$css = {
	hash: "svelte-444rfg",
	code: "\n	.outer.svelte-444rfg {\n		color: red;\n\n		.inner:where(.svelte-444rfg) {\n			color: blue;\n		}\n	}\n\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHVua25vd24pIiwic291cmNlcyI6WyIodW5rbm93bikiXSwic291cmNlc0NvbnRlbnQiOlsiPHN2ZWx0ZTpvcHRpb25zIGNzcz1cImluamVjdGVkXCIgLz5cblxuPHN0eWxlPlxuXHQub3V0ZXIge1xuXHRcdGNvbG9yOiByZWQ7XG5cblx0XHQuaW5uZXIge1xuXHRcdFx0Y29sb3I6IGJsdWU7XG5cdFx0fVxuXHR9XG48L3N0eWxlPlxuXG48ZGl2IGNsYXNzPVwib3V0ZXJcIj5cblx0PHNwYW4gY2xhc3M9XCJpbm5lclwiPmlubmVyPC9zcGFuPlxuPC9kaXY+XG4iXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IjtBQUdBLENBQUMsb0JBQU0sQ0FBQztBQUNSLEVBQUUsVUFBVTs7QUFFWixFQUFFLDRCQUFNLENBQUM7QUFDVCxHQUFHLFdBQVc7QUFDZDtBQUNBOyJ9 */"
};
function App($$renderer, $$props) {
	$$renderer.global.css.add($$css);
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div class="outer svelte-444rfg">`);
		$.push_element($$renderer, "div", 13, 0);
		$$renderer.push(`<span class="inner svelte-444rfg">`);
		$.push_element($$renderer, "span", 14, 1);
		$$renderer.push(`inner</span>`);
		$.pop_element();
		$$renderer.push(`</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
