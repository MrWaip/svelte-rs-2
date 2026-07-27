import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[9, 1]]);
var root_1 = $.add_locations($.from_html(`<!> <button>go</button>`, 1), App[$.FILENAME], [[11, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let n = $.tag($.state(1), "n");
	var d, e;
	var $$promises = $.run([async () => d = await $.async_derived(async () => (await $.track_reactivity_loss(Promise.resolve($.get(n))))(), "d", "(unknown):3:9"), async () => e = await $.async_derived(async () => (await $.track_reactivity_loss(Promise.resolve($.get(n) + 1)))(), "e", "(unknown):4:9")]);
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.first_child(fragment);
	$.async(node, [$$promises[0]], void 0, (node) => {
		var consequent = ($$anchor) => {
			let v;
			var promises = $.run([() => $.wait([$$promises[0], $$promises[1]]), () => v = $.tag($.derived(() => $.get(d) + $.get(e)), "v")]);
			var p = root();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, $.get(v)), void 0, void 0, [promises[1]]);
			$.append($$anchor, p);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.get(d)) $$render(consequent);
		}), "if", App, 7, 0);
	});
	var button = $.sibling(node, 2);
	$.delegated("click", button, function click() {
		return $.update(n);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
