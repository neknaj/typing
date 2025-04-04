// src/web/api.ts
async function file_get(file_path) {
  const response = await fetch(file_path);
  if (!response.ok) {
    return { error: response.status };
  }
  const text = await response.text();
  return { value: text };
}
function console_log(json) {
  console.log(...JSON.parse(json));
  return true;
}
export {
  console_log,
  file_get
};
