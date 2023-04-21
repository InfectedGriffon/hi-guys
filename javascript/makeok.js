import fs from 'fs';

let content = "let hey = {"
let closingBrackets = "}"
let callers = "."
for (const c of 'abcdefghijklmnopqrstuvwxyz') {
    content += c+":{"
    closingBrackets += "}"
    callers += c+"."
};
content += 'final:"woah"'
content += closingBrackets

content += "\n\nconsole.log(hey" + callers + "final)"


fs.writeFileSync('javascript/ok.js', content, e=>debug.log(e))