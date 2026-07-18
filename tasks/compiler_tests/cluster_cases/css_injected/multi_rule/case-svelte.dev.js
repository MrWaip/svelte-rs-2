App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div class="a svelte-1aej1md">a</div> <div class="b svelte-1aej1md">b</div>`, 1), App[$.FILENAME], [[13, 0], [14, 0]]);
const $$css = {
	hash: "svelte-1aej1md",
	code: "\n	.a.svelte-1aej1md {\n		color: red;\n	}\n\n	.b.svelte-1aej1md {\n		color: blue;\n	}\n\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHVua25vd24pIiwic291cmNlcyI6WyIodW5rbm93bikiXSwic291cmNlc0NvbnRlbnQiOlsiPHN2ZWx0ZTpvcHRpb25zIGNzcz1cImluamVjdGVkXCIgLz5cblxuPHN0eWxlPlxuXHQuYSB7XG5cdFx0Y29sb3I6IHJlZDtcblx0fVxuXG5cdC5iIHtcblx0XHRjb2xvcjogYmx1ZTtcblx0fVxuPC9zdHlsZT5cblxuPGRpdiBjbGFzcz1cImFcIj5hPC9kaXY+XG48ZGl2IGNsYXNzPVwiYlwiPmI8L2Rpdj5cbiJdLCJuYW1lcyI6W10sIm1hcHBpbmdzIjoiO0FBR0EsQ0FBQyxpQkFBRSxDQUFDO0FBQ0osRUFBRSxVQUFVO0FBQ1o7O0FBRUEsQ0FBQyxpQkFBRSxDQUFDO0FBQ0osRUFBRSxXQUFXO0FBQ2I7In0= */"
};
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	$.append_styles($$anchor, $$css);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	$.next(2);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
