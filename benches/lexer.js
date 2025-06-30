let a = [
  "let",
  "fn",
  "if",
  "then",
  "else",
  "end",
  "=",
  "!=",
  "==",
  ">",
  "<",
  ">=",
  "<=",
  "+",
  "-",
  "*",
  "/",
  "^",
  "%",
  "`",
  ",",
  "E",
  ":",
  "(",
  ")",
  "{",
  "}",
  "|",
  "[",
  "]",
  ";",
  "5",
  "name",
];

let s = "";

for (let i = 0; i < 100_00; i++) {
  for (let j = 0; j < 10; j++) {
    s += a[Math.floor(Math.random() * a.length)];
  }
  s += "\n";
}

console.log(s);
