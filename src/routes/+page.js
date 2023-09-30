import { BACKEND_BASE_URL } from "$lib/globals.js"
/** @type {import('./$types').PageLoad} */
export async function load({ fetch }) {
  let data = await fetch(BACKEND_BASE_URL + "/subject?_embed=grade").then(r => r.json());
  return { data };
}
