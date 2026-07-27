import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p> <button>bump</button>`, 1), App[$.FILENAME], [[7, 1], [8, 1]]);
var root_1 = $.add_locations($.from_html(`<!> <button>go</button>`, 1), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let n = $.tag($.state(1), "n");
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			let s;
			var promises = $.run([async () => s = $.tag($.state($.proxy((await $.track_reactivity_loss(Promise.resolve($.get(n))))())), "s")]);
			var fragment_1 = root();
			var p = $.first_child(fragment_1);
			var text = $.child(p, true);
			$.reset(p);
			var button = $.sibling(p, 2);
			$.template_effect(() => $.set_text(text, $.get(s)), void 0, void 0, [promises[0]]);
			$.delegated("click", button, function click() {
				return $.update(s);
			});
			$.append($$anchor, fragment_1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.get(n)) $$render(consequent);
		}), "if", App, 5, 0);
	}
	var button_1 = $.sibling(node, 2);
	$.delegated("click", button_1, function click_1() {
		return $.update(n);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
