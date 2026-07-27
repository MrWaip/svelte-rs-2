import * as $ from "svelte/internal/client";
var root = $.from_html(`<!doctype html=""/>`);
var root_1 = $.from_html(`<button>toggle</button> <!>`, 1);
export default function App($$anchor) {
	let show = $.state(true);
	var fragment = root_1();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	{
		var consequent = ($$anchor) => {
			var _doctype = root();
			$.append($$anchor, _doctype);
		};
		$.if(node, ($$render) => {
			if ($.get(show)) $$render(consequent);
		});
	}
	$.delegated("click", button, () => $.set(show, !$.get(show)));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
