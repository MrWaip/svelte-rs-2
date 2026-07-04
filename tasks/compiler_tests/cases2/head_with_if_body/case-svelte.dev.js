App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span>a</span>`), App[$.FILENAME], [[10, 4]]);
var root_1 = $.add_locations($.from_html(`<span>b</span>`), App[$.FILENAME], [[12, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let cond = false;
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	$.head("q2w0q4", ($$anchor) => {
		$.effect(() => {
			$.document.title = "t";
		});
	});
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
			if (cond) $$render(consequent);
			else $$render(alternate, -1);
		}), "if", App, 9, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
