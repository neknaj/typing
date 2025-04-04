export async function file_get(file_path) {
    const response = await fetch(file_path);
    if (!response.ok) {
        // Return an object representing the error (http status code)
        return { error: response.status };
    }
    // Return an object representing success with the file content
    const text = await response.text();
    return { value: text };
}

export function console_log(json) {
    console.log(...JSON.parse(json));
    return true;
}