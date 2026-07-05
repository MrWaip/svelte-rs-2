import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div class="svelte-19xqvng">`);
	$$renderer.push(`<style>
    .nested {
      color: red;
    }
  </style>`);
	$$renderer.push(` <p class="nested">inside div</p></div> `);
	if (true) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<style>
    span {
      color: green;
    }
  </style>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
