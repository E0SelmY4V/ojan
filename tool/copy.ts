import * as fsp from 'fs/promises';

const srcDir = __dirname + '/../answer/src/';
(async () => {
	let mainFile = ((await fsp.readFile(srcDir + 'main.rs')).toString());
	let modName = 'wrong!';
	for (let line of mainFile.split('\n')) {
		if (line.startsWith('use_as_now!')) {
			modName = line.slice('use_as_now!'.length + 1, -2);
			break;
		}
	}
	let r: string[] = [];
	for (let line of (await fsp.readFile(srcDir + 'lib.rs')).toString().split('\n')) {
		if (line.startsWith("#[cfg(test)]")  || line.startsWith('mod')) continue;
		r.push(line);
	}
	for (let line of (await fsp.readFile(srcDir + modName + '.rs')).toString().split('\n')) {
		if (line.startsWith('use ojan::*;')) {
			continue;
		}
		r.push(line);
	}
	console.log(r.join("\n"));
})();

