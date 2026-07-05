import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Box {
		#a = $.state($.proxy({ val: 0 }));
		#b = $.state(0);
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
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${box.a ?? ""} ${box.b ?? ""}`));
	$.append($$anchor, p);
	$.pop();
}
