import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[8, 1]]);
var root_1 = $.add_locations($.from_html(`<!> <button>go</button>`, 1), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let source = $.tag($.state($.proxy({
		x: 1,
		y: 2
	})), "source");
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const a = $.tag($.derived(() => $.get(source).x), "a");
			$.get(a);
			const computed_const = $.tag($.derived(() => {
				const { x, y } = $.get(source);
				return {
					x,
					y
				};
			}), "[@const]");
			$.get(computed_const);
			var p = root();
			var text = $.child(p);
			$.reset(p);
			$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(computed_const).x ?? ""}${$.get(computed_const).y ?? ""}`));
			$.append($$anchor, p);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.get(source)) $$render(consequent);
		}), "if", App, 5, 0);
	}
	var button = $.sibling(node, 2);
	$.delegated("click", button, function click() {
		return $.set(source, {
			x: 3,
			y: 4
		}, true);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
