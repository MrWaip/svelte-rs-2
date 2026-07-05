import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let obj = { field: { x: 0 } };
	let deep = { a: { b: { c: { x: 0 } } } };
	function script_ops() {
		obj.field.x = 1;
		obj.field.x += 2;
		obj.field.x++;
		++obj.field.x;
		obj.field.x &&= 3;
		deep.a.b.c.x = 4;
		deep.a.b.c.x += 5;
		deep.a.b.c.x++;
	}
	$$renderer.push(`<!---->${$.escape(obj.field.x)}
${$.escape(deep.a.b.c.x)}

${$.escape(obj.field.x = 1)}
${$.escape(obj.field.x += 2)}
${$.escape(obj.field.x++)}
${$.escape(++obj.field.x)}

${$.escape(deep.a.b.c.x = 4)}
${$.escape(deep.a.b.c.x++)} <button>run</button>`);
}
