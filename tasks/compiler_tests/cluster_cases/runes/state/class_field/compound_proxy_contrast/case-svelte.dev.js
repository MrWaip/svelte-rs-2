App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[19, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Box {
		#a = $.tag($.state($.proxy({ val: 0 })), "Box.#a");
		#b = $.tag($.state(0), "Box.#b");
		mix() {
			$.set(this.#a, $.get(this.#a) ?? { val: 1 }, true);
			$.set(this.#b, $.get(this.#b) + 1);
		}
		get a() {
			return $.get(this.#a)?.val;
		}
		get b() {
			return $.get(this.#b);
		}
	}
	const box = new Box();
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${box.a ?? ""} ${box.b ?? ""}`));
	$.append($$anchor, p);
	return $.pop($$exports);
}
