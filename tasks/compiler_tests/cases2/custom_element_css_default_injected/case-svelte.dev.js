App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p class="svelte-17bjde4">styled</p>`), App[$.FILENAME], [[7, 0]]);
const $$css = {
	hash: "svelte-17bjde4",
	code: "\n  p.svelte-17bjde4 {\n    color: red;\n  }\n\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHVua25vd24pIiwic291cmNlcyI6WyIodW5rbm93bikiXSwic291cmNlc0NvbnRlbnQiOlsiPHN2ZWx0ZTpvcHRpb25zIGN1c3RvbUVsZW1lbnQ9XCJteS1zdHlsZWRcIiAvPlxuPHN0eWxlPlxuICBwIHtcbiAgICBjb2xvcjogcmVkO1xuICB9XG48L3N0eWxlPlxuPHA+c3R5bGVkPC9wPlxuIl0sIm5hbWVzIjpbXSwibWFwcGluZ3MiOiI7QUFFQSxFQUFFLGdCQUFDLENBQUM7QUFDSixJQUFJLFVBQVU7QUFDZDsifQ== */"
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
customElements.define("my-styled", $.create_custom_element(App, {}, [], [], { mode: "open" }));
