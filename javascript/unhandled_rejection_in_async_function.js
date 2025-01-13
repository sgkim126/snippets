const process = require('node:process');


process.on('unhandledRejection', (reason, promise) => {
    console.log({reason, promise});
});


async function foo() {
    async function f1() {
        throw "error from f1";
    }
    async function f2() {
        throw "error from f2";
    }
    async function f3() {
        throw "error from f3";
    }
    const asyncP1 = f1();
    const asyncP2 = f2();
    const asnycP3 = f3();
    return {
        p1: await asyncP1,
        p2: await asyncP2,
        p3: await asyncP3
    };
}

(async () => {
    try {
        const result = await foo();
        console.log({result});
    } catch (err) {
        console.log({err});
    }
})();
