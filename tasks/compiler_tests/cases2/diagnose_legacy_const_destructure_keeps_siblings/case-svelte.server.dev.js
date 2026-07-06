App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let src = $$props["src"];
		const tmp = src, a = tmp.a, b = tmp.b, c = tmp.c;
		const tmp_1 = src.list, $$array = $.to_array(tmp_1, 2), d = $$array[0], e = $$array[1];
		const tmp_2 = src.nested, f = tmp_2.f, h = tmp_2.g.h, j = tmp_2.g.i;
		const tmp_3 = src.mixed, $$array_1 = $.to_array(tmp_3.k, 2), l = $$array_1[0], m = $$array_1[1];
		const tmp_4 = src.arrobj, $$array_2 = $.to_array(tmp_4, 2), n = $$array_2[0], o = $$array_2[1].o, p = $$array_2[1].p;
		const tmp_5 = src.renamed, r = tmp_5.q, t = tmp_5.s;
		function mutate() {
			b.x = 1;
			e.x = 2;
			h.x = 3;
			l.x = 4;
			o.x = 5;
			r.x = 6;
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 21, 0);
		$$renderer.push(`${$.escape(a)} ${$.escape(b.x)} ${$.escape(c)}
    ${$.escape(d)} ${$.escape(e.x)}
    ${$.escape(f)} ${$.escape(h.x)} ${$.escape(j)}
    ${$.escape(l.x)} ${$.escape(m)}
    ${$.escape(n)} ${$.escape(o.x)} ${$.escape(p)}
    ${$.escape(r.x)} ${$.escape(t)}</button>`);
		$.pop_element();
		$.bind_props($$props, { src });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
