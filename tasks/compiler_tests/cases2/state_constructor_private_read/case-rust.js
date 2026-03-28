import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Timer {
		#elapsed = $.state(0);
		tick() {
			$.set(this.#elapsed, $.get(this.#elapsed) + 1);
		}
		get display() {
			return $.get(this.#elapsed);
		}
	}
	let t = new Timer();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, t.display));
	$.append($$anchor, p);
	$.pop();
}
