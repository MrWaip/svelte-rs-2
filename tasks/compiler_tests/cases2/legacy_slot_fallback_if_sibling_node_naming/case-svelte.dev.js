import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[9, 12]]);
var root_1 = $.add_locations($.from_html(`<!> <div>tail</div>`, 1), App[$.FILENAME], [[11, 8]]);
var root_2 = $.add_locations($.from_html(`<li><!></li>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let show = $.prop($$props, "show", 8);
	let value = $.prop($$props, "value", 8);
	var $$exports = { ...$.legacy_api() };
	var li = root_2();
	var node = $.child(li);
	$.slot(node, $$props, "item", {}, ($$anchor) => {
		var fragment = root_1();
		var node_1 = $.first_child(fragment);
		{
			var consequent = ($$anchor) => {
				var span = root();
				var text = $.child(span, true);
				$.reset(span);
				$.template_effect(() => $.set_text(text, value()));
				$.append($$anchor, span);
			};
			$.add_svelte_meta(() => $.if(node_1, ($$render) => {
				if (show()) $$render(consequent);
			}), "if", App, 8, 8);
		}
		$.next(2);
		$.append($$anchor, fragment);
	});
	$.reset(li);
	$.append($$anchor, li);
	return $.pop($$exports);
}
