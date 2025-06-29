let a = ["let","fn","if", "then", "else", "end", "=", "!=", "==", ">", "<", ">=", "<=", "+", "-", "*", "/", "^", "%", "`", ",", "E", ":", "(", ")", "{", "}", "|","[","]",";"];

let s = "";

for (let i = 0; i < 100_00; i++) {
	s += a[Math.round(Math.random() * a.length)] + "\n"
}

console.log(s)