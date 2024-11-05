export async function get_raw(url) {
  const response = await fetch(url);
  return new Uint8Array(await response.arrayBuffer());
}

export async function post_raw(url, body) {
  const response = await fetch(url, {
    "method": "POST",
    "body": body,
  });
  return new Uint8Array(await response.arrayBuffer());
}
