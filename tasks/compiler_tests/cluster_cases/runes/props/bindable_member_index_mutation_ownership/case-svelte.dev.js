App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let rows = $.prop($$props, "rows", 31, () => $.tag_proxy($.proxy([]), "rows"));
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var input = root();
			$.remove_input_defaults(input);
			$.validate_binding("bind:checked={rows[0].check}", [], () => rows()[0], () => "check", 6, 24);
			$.bind_checked(input, function get() {
				return rows()[0].check;
			}, function set($$value) {
				$$ownership_validator.mutation("rows", [
					"rows",
					0,
					"check"
				], rows(rows()[0].check = $$value, true), 6, 38);
			});
			$.append($$anchor, input);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (rows().length) $$render(consequent);
		}), "if", App, 5, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
