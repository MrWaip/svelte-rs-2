import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>reset</button>`);
var root_1 = $.from_html(`<p>a</p>`);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const pending = ($$anchor) => {
			$.next();
			var text = $.text("pending");
			$.append($$anchor, text);
		};
		const failed = ($$anchor, _ = $.noop, reset = $.noop) => {
			var button = root();
			$.delegated("click", button, function(...$$args) {
				reset()?.apply(this, $$args);
			});
			$.append($$anchor, button);
		};
		$.boundary(node, {
			pending,
			failed
		}, ($$anchor) => {
			var p = root_1();
			$.append($$anchor, p);
		});
	}
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
