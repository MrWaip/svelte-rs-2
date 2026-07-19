App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div class="box svelte-bulewn">box</div>`), App[$.FILENAME], [[10, 0]]);
const $$css = {
	hash: "svelte-bulewn",
	code: "\n	.box.svelte-bulewn {\n		--gap: 10px;\n		color: red;\n	}\n\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHVua25vd24pIiwic291cmNlcyI6WyIodW5rbm93bikiXSwic291cmNlc0NvbnRlbnQiOlsiPHN2ZWx0ZTpvcHRpb25zIGNzcz1cImluamVjdGVkXCIgLz5cblxuPHN0eWxlPlxuXHQuYm94IHtcblx0XHQtLWdhcDogMTBweDtcblx0XHRjb2xvcjogcmVkO1xuXHR9XG48L3N0eWxlPlxuXG48ZGl2IGNsYXNzPVwiYm94XCI+Ym94PC9kaXY+XG4iXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IjtBQUdBLENBQUMsa0JBQUksQ0FBQztBQUNOLEVBQUUsV0FBVztBQUNiLEVBQUUsVUFBVTtBQUNaOyJ9 */"
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
