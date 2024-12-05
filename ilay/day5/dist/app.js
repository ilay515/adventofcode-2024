"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const fs_1 = __importDefault(require("fs"));
const input = fs_1.default.readFileSync("src/input.txt", { encoding: 'utf8', flag: 'r' });
const [rawMapping, sections] = input.split("\n\n", 2);
const mapping = {};
rawMapping.split("\n").forEach((rule) => {
    const [first, second] = rule.split("|", 2);
    if (mapping[first]) {
        mapping[first].push(second);
    }
    else {
        mapping[first] = [second];
    }
});
const isSectionValid = (section) => {
    for (let i = 1; i < section.length; i++) {
        const currentPage = section[i];
        if (!mapping[currentPage])
            continue;
        for (let j = 0; j < i; j++) {
            if (mapping[currentPage].includes(section[j])) {
                return false;
            }
        }
    }
    return true;
};
const validSections = sections
    .split("\n")
    .map((section) => section.split(","))
    .filter(isSectionValid);
const result = validSections
    .map((section) => Number(section[Math.floor(section.length / 2)]))
    .reduce((a, b) => a + b, 0);
console.log(result);
//# sourceMappingURL=app.js.map