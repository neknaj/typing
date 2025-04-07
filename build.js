const esbuild = require('esbuild');
const { exec } = require('child_process');
const fs = require('fs');
const util = require('util');
const path = require('path');
const https = require('https');

const execPromise = util.promisify(exec);

function makedir() {
    const destPath = './dist';
    const destDir = path.resolve(destPath);
    if (!fs.existsSync(destDir)) {
        fs.mkdirSync(destDir, { recursive: true });
    }
    console.log('Directory created:', destDir);
}

function buildTSapi() {
    esbuild.build({
        entryPoints: ['./src/web/api.ts'], // エントリーポイント
        tsconfig: './tsconfig.json', // tsconfig.jsonのパス
        outfile: './src/web/api.js',    // 出力先
        bundle: true,                   // 依存関係をバンドル
        // minify: true,                   // 圧縮
        minify: false,                   // 圧縮
        // sourcemap: false,                // ソースマップ生成
        target: ['esnext'],             // トランスパイルのターゲット
        loader: { '.ts': 'ts' },        // TypeScriptを処理
        format: 'esm',  // 出力形式をESモジュールにする
    }).then(() => {
        console.log('TS Build succeeded!');
    }).catch(() => process.exit(1));
}

function buildTS() {
    esbuild.build({
        entryPoints: ['./src/web/index.ts'], // エントリーポイント
        tsconfig: './tsconfig.json', // tsconfig.jsonのパス
        outfile: './dist/index.js',    // 出力先
        bundle: true,                   // 依存関係をバンドル
        // minify: true,                   // 圧縮
        minify: false,                   // 圧縮
        // sourcemap: false,                // ソースマップ生成
        target: ['esnext'],             // トランスパイルのターゲット
        loader: { '.ts': 'ts' },        // TypeScriptを処理
        format: 'esm',  // 出力形式をESモジュールにする
    }).then(() => {
        console.log('TS Build succeeded!');
    }).catch(() => process.exit(1));
}

async function buildRust(args) {
    // try {
    //     const { stdout, stderr } = await execPromise('cargo test --features web');
    //     process.stdout.write(stdout);
    //     if (stderr) {
    //         process.stderr.write(stderr);
    //     }
    // } catch (error) {
    //     // エラーオブジェクトから終了コードを取得
    //     process.stderr.write(error.message+"\n");
    //     process.stderr.write(error.statusCode+"\n");
    //     throw error.statusCode;
    // }
    try {
        let flag = args.includes("--release")? "--release" : "--debug";
        // wasm-packコマンドを実行
        const { stdout, stderr } = await execPromise('wasm-pack build '+flag+' --target web --no-default-features --features web');
        process.stdout.write(stdout);
        if (stderr) {
            process.stderr.write(stderr);
        }
        console.log(flag)
        console.log('Wasm build complete!');
        return true;
    } catch (error) {
        // エラーオブジェクトから終了コードを取得
        process.stderr.write(error.message+"\n");
        process.stderr.write(error.statusCode+"\n");
        throw error.statusCode;
    }
}


function copyFiles(files) {
    // 各ファイルをコピーするプロミスの配列を作成
    const copyPromises = files.map((file) => {
        const sourcePath = path.resolve(file[0]);
        const destinationPath = path.resolve(file[1]);
        return fs.promises.copyFile(sourcePath, destinationPath);
    });
    return Promise.all(copyPromises);
}

async function copyDirectory(source, destination) {
    await fs.promises.mkdir(destination, { recursive: true });
    const entries = await fs.promises.readdir(source, { withFileTypes: true });
    for (let entry of entries) {
        const srcPath = path.join(source, entry.name);
        const destPath = path.join(destination, entry.name);
        if (entry.isDirectory()) {
            await copyDirectory(srcPath, destPath);
        } else {
            await fs.promises.copyFile(srcPath, destPath);
        }
    }
}

async function getFile(savePath,url) {
    // if (fs.existsSync(savePath)) return false; // 既に存在するならダウンロードしない
    await new Promise((resolve, reject) => {
        https.get(url, (res) => {
            if (res.statusCode === 200) {
                const file = fs.createWriteStream(savePath);
                res.pipe(file);

                file.on('finish', () => {
                    resolve('File downloaded and saved');
                });

                file.on('error', (err) => {
                    reject(`Error writing to file: ${err.message}`);
                });
            } else {
                reject(`Failed to download file. Status code: ${res.statusCode}`);
            }
        }).on('error', (err) => {
            reject(`Error: ${err.message}`);
        });
    });
    return true;
}

async function main() {
    const args = process.argv.slice(2);
    makedir();
    await buildTSapi();
    if (!args.includes("-tsonly")) {
        await buildRust(args);
    }
    await copyFiles([
        [
            "./pkg/typing_lib.js",
            "./src/web/typing_lib.js"
        ],
        [
            "./pkg/typing_lib.d.ts",
            "./src/web/typing_lib.d.ts"
        ],
        [
            "./pkg/typing_lib_bg.wasm",
            "./dist/typing_lib_bg.wasm"
        ],
    ]);
    await copyDirectory('./pkg/snippets', './src/web/snippets');
    await getFile('./src/web/cdom.ts','https://raw.githubusercontent.com/neknaj/cDom/50a65673454c7286830f0d131f0512ddf46a3844/cdom_module.ts');
    // if (await getFile('./src/web/layout.js','https://raw.githubusercontent.com/neknaj/webSplitLayout/c7e1c52cb37a8bfbf9968b825c05a2e9924ca88e/type1/layout.js')) {
    //     fs.readFile('./src/web/layout.js', 'utf8', (err, data) => {
    //         const updatedData = "import { elm } from './cdom.js';\n\n" + data + "\n\nexport { initlayout };";
    //         fs.writeFile('./src/web/layout.js', updatedData, (err) => {});
    //     });
    // };
    await getFile('./dist/layout.css','https://raw.githubusercontent.com/neknaj/webSplitLayout/c7e1c52cb37a8bfbf9968b825c05a2e9924ca88e/type1/layout.css');
    await buildTS();
    await copyFiles([
        [
            "./src/web/index.html",
            "./dist/index.html"
        ],
        [
            "./src/web/index.css",
            "./dist/index.css"
        ],
    ]);
    await copyDirectory('./examples', './dist/examples');
    await copyDirectory('./layouts', './dist/layouts');
}

main()