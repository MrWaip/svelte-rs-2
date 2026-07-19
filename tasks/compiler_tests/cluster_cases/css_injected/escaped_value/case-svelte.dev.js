App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span class="icon svelte-p6hspt"></span>`), App[$.FILENAME], [[9, 0]]);
const $$css = {
	hash: "svelte-p6hspt",
	code: "\n	.icon.svelte-p6hspt::before {\n		content: \"\\ff\";\n	}\n\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHVua25vd24pIiwic291cmNlcyI6WyIodW5rbm93bikiXSwic291cmNlc0NvbnRlbnQiOlsiPHN2ZWx0ZTpvcHRpb25zIGNzcz1cImluamVjdGVkXCIgLz5cblxuPHN0eWxlPlxuXHQuaWNvbjo6YmVmb3JlIHtcblx0XHRjb250ZW50OiBcIlxcZmZcIjtcblx0fVxuPC9zdHlsZT5cblxuPHNwYW4gY2xhc3M9XCJpY29uXCI+PC9zcGFuPlxuIl0sIm5hbWVzIjpbXSwibWFwcGluZ3MiOiI7QUFHQSxDQUFDLG1CQUFLLFFBQVEsQ0FBQztBQUNmLEVBQUUsY0FBYztBQUNoQjsifQ== */"
};
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	$.append_styles($$anchor, $$css);
	var $$exports = { ...$.legacy_api() };
	var span = root();
	$.append($$anchor, span);
	return $.pop($$exports);
}
