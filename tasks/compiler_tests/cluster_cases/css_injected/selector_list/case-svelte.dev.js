App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div class="a svelte-limxtm">a</div> <div class="b svelte-limxtm">b</div>`, 1), App[$.FILENAME], [[10, 0], [11, 0]]);
const $$css = {
	hash: "svelte-limxtm",
	code: "\n	.a.svelte-limxtm,\n	.b.svelte-limxtm {\n		color: red;\n	}\n\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHVua25vd24pIiwic291cmNlcyI6WyIodW5rbm93bikiXSwic291cmNlc0NvbnRlbnQiOlsiPHN2ZWx0ZTpvcHRpb25zIGNzcz1cImluamVjdGVkXCIgLz5cblxuPHN0eWxlPlxuXHQuYSxcblx0LmIge1xuXHRcdGNvbG9yOiByZWQ7XG5cdH1cbjwvc3R5bGU+XG5cbjxkaXYgY2xhc3M9XCJhXCI+YTwvZGl2PlxuPGRpdiBjbGFzcz1cImJcIj5iPC9kaXY+XG4iXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IjtBQUdBLENBQUMsZ0JBQUU7QUFDSCxDQUFDLGdCQUFFLENBQUM7QUFDSixFQUFFLFVBQVU7QUFDWjsifQ== */"
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
