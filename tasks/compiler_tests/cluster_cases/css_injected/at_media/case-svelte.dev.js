App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div class="box svelte-1ngs2ez">box</div>`), App[$.FILENAME], [[11, 0]]);
const $$css = {
	hash: "svelte-1ngs2ez",
	code: "\n	@media (min-width: 100px) {\n		.box.svelte-1ngs2ez {\n			color: red;\n		}\n	}\n\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHVua25vd24pIiwic291cmNlcyI6WyIodW5rbm93bikiXSwic291cmNlc0NvbnRlbnQiOlsiPHN2ZWx0ZTpvcHRpb25zIGNzcz1cImluamVjdGVkXCIgLz5cblxuPHN0eWxlPlxuXHRAbWVkaWEgKG1pbi13aWR0aDogMTAwcHgpIHtcblx0XHQuYm94IHtcblx0XHRcdGNvbG9yOiByZWQ7XG5cdFx0fVxuXHR9XG48L3N0eWxlPlxuXG48ZGl2IGNsYXNzPVwiYm94XCI+Ym94PC9kaXY+XG4iXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IjtBQUdBLENBQUMsMEJBQTBCO0FBQzNCLEVBQUUsbUJBQUksQ0FBQztBQUNQLEdBQUcsVUFBVTtBQUNiO0FBQ0E7In0= */"
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
