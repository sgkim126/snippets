async function a() {
  throw "rejected";
}

async function b() {
  try {
    console.log("before");
    await a();
    console.log("after");
  } catch (e) {
    console.log("catch", e);
  }
  console.log("end");
}

b();
