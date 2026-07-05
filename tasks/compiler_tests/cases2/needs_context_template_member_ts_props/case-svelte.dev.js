App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { Kind } from "./kinds";
var root = $.add_locations($.from_html(`<span>A</span>`), App[$.FILENAME], [[11, 4]]);
var root_1 = $.add_locations($.from_html(`<span>B</span>`), App[$.FILENAME], [[13, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var span = root();
			$.append($$anchor, span);
		};
		var alternate = ($$anchor) => {
			var span_1 = root_1();
			$.append($$anchor, span_1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.strict_equals($$props.item.kind, Kind.A)) $$render(consequent);
			else $$render(alternate, -1);
		}), "if", App, 10, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
