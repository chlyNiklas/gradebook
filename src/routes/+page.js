/** @type {import('./$types').PageLoad} */
export async function load({ fetch }) {
  let data = await fetch("http://localhost:3000/subject?_embed=grade").then(r => r.json());
  return { data };
}
