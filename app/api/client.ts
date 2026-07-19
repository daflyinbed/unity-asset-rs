import createClient from "openapi-fetch";
import type { paths } from "./schema";

const IPC_ORIGIN = "http://rust.internal";

const ipcFetch: typeof fetch = async (input, init) => {
  const request = new Request(input, init);
  const url = new URL(request.url, IPC_ORIGIN);
  const result = await window.electron.invoke("api:request", {
    method: request.method,
    path: `${url.pathname}${url.search}`,
    headers: Object.fromEntries(request.headers.entries()),
    body: request.body ? await request.text() : undefined,
  });
  if (!result.ok) throw new Error(result.error.message);
  return new Response(result.value.body, {
    status: result.value.status,
    headers: result.value.headers,
  });
};

export const api = createClient<paths>({
  baseUrl: IPC_ORIGIN,
  fetch: ipcFetch,
});
