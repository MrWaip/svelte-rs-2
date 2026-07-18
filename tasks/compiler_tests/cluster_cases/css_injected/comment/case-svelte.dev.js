App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div class="box svelte-kpvy84">box</div>`), App[$.FILENAME], [[10, 0]]);
const $$css = {
	hash: "svelte-kpvy84",
	code: "\n	/* a comment */\n	.box.svelte-kpvy84 {\n		color: red;\n	}\n\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHVua25vd24pIiwic291cmNlcyI6WyIodW5rbm93bikiXSwic291cmNlc0NvbnRlbnQiOlsiPHN2ZWx0ZTpvcHRpb25zIGNzcz1cImluamVjdGVkXCIgLz5cblxuPHN0eWxlPlxuXHQvKiBhIGNvbW1lbnQgKi9cblx0LmJveCB7XG5cdFx0Y29sb3I6IHJlZDtcblx0fVxuPC9zdHlsZT5cblxuPGRpdiBjbGFzcz1cImJveFwiPmJveDwvZGl2PlxuIl0sIm5hbWVzIjpbXSwibWFwcGluZ3MiOiI7QUFHQTtBQUNBLENBQUMsa0JBQUksQ0FBQztBQUNOLEVBQUUsVUFBVTtBQUNaOyJ9 */"
};
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	$.append_styles($$anchor, $$css);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.append($$anchor, div);
	return $.pop($$exports);
}
