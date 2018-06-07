let a = async () => {
  throw "a rejected";
};
let b = async () => {
  await a();
  return "b returned";
}

b().then((a) => { console.log("resolve", a); }, (a) => { console.log("reject", a); });
