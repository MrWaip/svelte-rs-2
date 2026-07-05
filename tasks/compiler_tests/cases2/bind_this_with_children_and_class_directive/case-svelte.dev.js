App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span>x</span>`), App[$.FILENAME], [[11, 8]]);
var root_1 = $.add_locations($.from_html(`<div><!></div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let dynamicEl;
	let counter = 0;
	var $$exports = { ...$.legacy_api() };
	var div = root_1();
	$.set_class(div, 1, "", null, {}, { state: counter > 0 });
	var node = $.child(div);
	{
		var consequent = ($$anchor) => {
			var span = root();
			$.append($$anchor, span);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (counter) $$render(consequent);
		}), "if", App, 10, 4);
	}
	$.reset(div);
	$.bind_this(div, ($$value) => dynamicEl = $$value, () => dynamicEl);
	$.append($$anchor, div);
	return $.pop($$exports);
}
