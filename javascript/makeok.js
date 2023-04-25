import fs from 'fs';

let content = "let hey = {"
let closingBrackets = "}"
let callers = "."

for (const letter of 'abcdefghijklmnopqrstuvwxyz') {
    content += letter+":{"
    closingBrackets += "}"
    callers += letter+"."
};

content += 'final:"woah"'
        + closingBrackets
        + "\n\nconsole.log(hey"
        + callers
        + "final)"

fs.writeFileSync('javascript/ok.js', content, e=>debug.log(e))