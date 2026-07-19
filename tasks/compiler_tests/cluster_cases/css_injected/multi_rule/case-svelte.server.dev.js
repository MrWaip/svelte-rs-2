App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
const $$css = {
	hash: "svelte-1aej1md",
	code: "\n	.a.svelte-1aej1md {\n		color: red;\n	}\n\n	.b.svelte-1aej1md {\n		color: blue;\n	}\n\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHVua25vd24pIiwic291cmNlcyI6WyIodW5rbm93bikiXSwic291cmNlc0NvbnRlbnQiOlsiPHN2ZWx0ZTpvcHRpb25zIGNzcz1cImluamVjdGVkXCIgLz5cblxuPHN0eWxlPlxuXHQuYSB7XG5cdFx0Y29sb3I6IHJlZDtcblx0fVxuXG5cdC5iIHtcblx0XHRjb2xvcjogYmx1ZTtcblx0fVxuPC9zdHlsZT5cblxuPGRpdiBjbGFzcz1cImFcIj5hPC9kaXY+XG48ZGl2IGNsYXNzPVwiYlwiPmI8L2Rpdj5cbiJdLCJuYW1lcyI6W10sIm1hcHBpbmdzIjoiO0FBR0EsQ0FBQyxpQkFBRSxDQUFDO0FBQ0osRUFBRSxVQUFVO0FBQ1o7O0FBRUEsQ0FBQyxpQkFBRSxDQUFDO0FBQ0osRUFBRSxXQUFXO0FBQ2I7In0= */"
};
function App($$renderer, $$props) {
	$$renderer.global.css.add($$css);
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div class="a svelte-1aej1md">`);
		$.push_element($$renderer, "div", 13, 0);
		$$renderer.push(`a</div>`);
		$.pop_element();
		$$renderer.push(` <div class="b svelte-1aej1md">`);
		$.push_element($$renderer, "div", 14, 0);
		$$renderer.push(`b</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
