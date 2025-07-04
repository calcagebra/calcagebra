let s = "";

function generate_expression() {
  let expr = "";

  let a = ["+", "-", "*", "/", "^", "%"];
  let b = ["name", "5.0", "5", "(5 + 5i)"];

  expr +=
    b[Math.floor(Math.random() * b.length)] +
    a[Math.floor(Math.random() * a.length)] +
    b[Math.floor(Math.random() * b.length)] +
    a[Math.floor(Math.random() * a.length)] +
    b[Math.floor(Math.random() * b.length)];

  if (Math.random() < 0.5) {
    expr = `|${expr}|`
  }

  return expr
}

for (let i = 0; i < 100_00; i++) {
  let x = Math.floor(Math.random() * 3);

  if (x == 0) {
    s += `let name: C = C(${generate_expression()})`;
  } else if (x === 1) {
    s += `fn name(x: C): C = C(${generate_expression()})`
  } else {
    s +=`print(C(${generate_expression()}))`
  }

  s += "\n"
}

console.log(s)