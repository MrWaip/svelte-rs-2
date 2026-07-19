App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div class="outer svelte-444rfg"><span class="inner svelte-444rfg">inner</span></div>`), App[$.FILENAME], [[
	13,
	0,
	[[14, 1]]
]]);
const $$css = {
	hash: "svelte-444rfg",
	code: "\n	.outer.svelte-444rfg {\n		color: red;\n\n		.inner:where(.svelte-444rfg) {\n			color: blue;\n		}\n	}\n\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHVua25vd24pIiwic291cmNlcyI6WyIodW5rbm93bikiXSwic291cmNlc0NvbnRlbnQiOlsiPHN2ZWx0ZTpvcHRpb25zIGNzcz1cImluamVjdGVkXCIgLz5cblxuPHN0eWxlPlxuXHQub3V0ZXIge1xuXHRcdGNvbG9yOiByZWQ7XG5cblx0XHQuaW5uZXIge1xuXHRcdFx0Y29sb3I6IGJsdWU7XG5cdFx0fVxuXHR9XG48L3N0eWxlPlxuXG48ZGl2IGNsYXNzPVwib3V0ZXJcIj5cblx0PHNwYW4gY2xhc3M9XCJpbm5lclwiPmlubmVyPC9zcGFuPlxuPC9kaXY+XG4iXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IjtBQUdBLENBQUMsb0JBQU0sQ0FBQztBQUNSLEVBQUUsVUFBVTs7QUFFWixFQUFFLDRCQUFNLENBQUM7QUFDVCxHQUFHLFdBQVc7QUFDZDtBQUNBOyJ9 */"
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
