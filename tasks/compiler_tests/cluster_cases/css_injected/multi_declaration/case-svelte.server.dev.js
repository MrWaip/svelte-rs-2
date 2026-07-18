App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
const $$css = {
	hash: "svelte-354c5f",
	code: "\n	.box.svelte-354c5f {\n		color: red;\n		width: 10px;\n	}\n\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHVua25vd24pIiwic291cmNlcyI6WyIodW5rbm93bikiXSwic291cmNlc0NvbnRlbnQiOlsiPHN2ZWx0ZTpvcHRpb25zIGNzcz1cImluamVjdGVkXCIgLz5cblxuPHN0eWxlPlxuXHQuYm94IHtcblx0XHRjb2xvcjogcmVkO1xuXHRcdHdpZHRoOiAxMHB4O1xuXHR9XG48L3N0eWxlPlxuXG48ZGl2IGNsYXNzPVwiYm94XCI+Ym94PC9kaXY+XG4iXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IjtBQUdBLENBQUMsa0JBQUksQ0FBQztBQUNOLEVBQUUsVUFBVTtBQUNaLEVBQUUsV0FBVztBQUNiOyJ9 */"
};
function App($$renderer, $$props) {
	$$renderer.global.css.add($$css);
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div class="box svelte-354c5f">`);
		$.push_element($$renderer, "div", 10, 0);
		$$renderer.push(`box</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
