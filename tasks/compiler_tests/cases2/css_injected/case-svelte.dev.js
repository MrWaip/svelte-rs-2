App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p class="svelte-1a7i8ec">styled</p>`), App[$.FILENAME], [[9, 0]]);
const $$css = {
	hash: "svelte-1a7i8ec",
	code: "\n	p.svelte-1a7i8ec {\n		color: red;\n	}\n\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHVua25vd24pIiwic291cmNlcyI6WyIodW5rbm93bikiXSwic291cmNlc0NvbnRlbnQiOlsiPHN2ZWx0ZTpvcHRpb25zIGNzcz1cImluamVjdGVkXCIgLz5cblxuPHN0eWxlPlxuXHRwIHtcblx0XHRjb2xvcjogcmVkO1xuXHR9XG48L3N0eWxlPlxuXG48cD5zdHlsZWQ8L3A+XG4iXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IjtBQUdBLENBQUMsZ0JBQUMsQ0FBQztBQUNILEVBQUUsVUFBVTtBQUNaOyJ9 */"
};
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	$.append_styles($$anchor, $$css);
	var $$exports = { ...$.legacy_api() };
	var p = root();
	$.append($$anchor, p);
	return $.pop($$exports);
}
