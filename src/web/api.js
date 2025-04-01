export async function file_get(file_path) {
    // fetch APIでファイルを読み込みます
    const response = await fetch(file_path);
    if (!response.ok) {
        throw new Error(`Error fetching ${file_path}: ${response.statusText}`);
    }
    // ファイル内容をテキストとして返します
    return await response.text();
}